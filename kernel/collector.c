#include <linux/mutex.h>
#include <linux/module.h>
#include <net/genetlink.h>

#ifdef pr_fmt
#undef pr_fmt
#endif
#define pr_fmt(text) KBUILD_MODNAME ": " text

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Sergei Koparov");
MODULE_DESCRIPTION("Collector kernel module");

#define FAMILY_NAME "collector"

enum COLLECTOR_COMMAND
{
    COLLECTOR_COMMAND_HIDE,
    COLLECTOR_COMMAND_UNHIDE,
    COLLECTOR_COMMAND_UNINSTALL,
    COLLECTOR_COMMAND_COUNT
};

enum COLLECTOR_ATTRIBUTE
{
    COLLECTOR_ATTRIBUTE_UNSPEC,
    COLLECTOR_ATTRIBUTE_MSG,
    COLLECTOR_ATTRIBUTE_COUNT
};

enum COLLECTOR_RESULT
{
    COLLECTOR_OK,
    COLLECTOR_ERROR_SYSTEM,
    COLLECTOR_ERROR_PROTOCOL,
    COLLECTOR_ERROR_NO_PATH,
    COLLECTOR_ERROR_PATH_NOT_FOUND,
    COLLECTOR_ERROR_OTHER
};

struct mutex mtx;

int hide_cb(struct sk_buff *sender_skb, struct genl_info *info);
int unhide_cb(struct sk_buff *sender_skb, struct genl_info *info);
int uninstall_cb(struct sk_buff *sender_skb, struct genl_info *info);

struct genl_ops collector_ops[COLLECTOR_COMMAND_COUNT] =
{
    {
        .cmd = COLLECTOR_COMMAND_HIDE,
        .flags = 0,
        .internal_flags = 0,
        .doit = hide_cb,
        .dumpit = NULL,
        .start = NULL,
        .done = NULL,
        .validate = 0
    },
    {
        .cmd = COLLECTOR_COMMAND_UNHIDE,
        .flags = 0,
        .internal_flags = 0,
        .doit = unhide_cb,
        .dumpit = NULL,
        .start = NULL,
        .done = NULL,
        .validate = 0
    },
    {
        .cmd = COLLECTOR_COMMAND_UNINSTALL,
        .flags = 0,
        .internal_flags = 0,
        .doit = uninstall_cb,
        .dumpit = NULL,
        .start = NULL,
        .done = NULL,
        .validate = 0
    }
};

static struct nla_policy collector_policy[COLLECTOR_ATTRIBUTE_COUNT] = {
    [COLLECTOR_ATTRIBUTE_UNSPEC] = {.type = NLA_UNSPEC},
    [COLLECTOR_ATTRIBUTE_MSG] = {.type = NLA_NUL_STRING}
};

static struct genl_family family_descr = {
        .id = 0, // auto id
        .hdrsize = 0,
        .name = FAMILY_NAME,
        .version = 1,
        .ops = collector_ops,
        .n_ops = COLLECTOR_COMMAND_COUNT,
        .policy = collector_policy,
        .maxattr = COLLECTOR_ATTRIBUTE_COUNT,
        .module = THIS_MODULE,
        .parallel_ops = 0,
        .netnsok = 0,
        .pre_doit = NULL,
        .post_doit = NULL
};

enum COLLECTOR_RESULT hide_path(char* path)
{
    pr_info("Hide path: %s\n", path);
    return COLLECTOR_OK;
 }

enum COLLECTOR_RESULT unhide_path(char* path)
{
    pr_info("Unhide path: %s\n", path);
    return COLLECTOR_OK;
}

enum COLLECTOR_RESULT uninstall(void)
{
    pr_info("Uninstall\n");
    return COLLECTOR_OK;
}

char* get_path(struct genl_info* info)
{
    char* path;
    struct nlattr* attr;

    if (!info)
    {
        pr_err("Info is null\n");
        return NULL;
    }

    attr = info->attrs[COLLECTOR_ATTRIBUTE_MSG];
    if (!attr)
    {
        pr_err("Path not found in attrs\n");
        return NULL;
    }

    path = (char*)nla_data(attr);
    if (path == NULL)
    {
        pr_err("Path is empty\n");
        return NULL;
    }

    return path;
}

int send_reply(struct genl_info* info, enum COLLECTOR_COMMAND command, enum COLLECTOR_RESULT result)
{
    int res;
    void* msg_head;
    struct sk_buff* reply;

    if (!info)
    {
        pr_err("Info is null\n");
        return -1;
    }

    reply = genlmsg_new(NLMSG_GOODSIZE, GFP_KERNEL);
    if (!reply)
    {
        pr_err("Failed to create reply buff\n");
        return -ENOMEM;
    }

    msg_head = genlmsg_put(reply,
                         info->snd_portid,
                         info->snd_seq + 1,
                         &family_descr,
                         0, // flags
                         command);

    if (!msg_head)
    {
        pr_err("Failed to create msg head\n");
        return -ENOMEM;
    }

    res = nla_put_u8(reply, COLLECTOR_ATTRIBUTE_MSG, result);
    if (res != 0)
    {
        pr_err("Failed to put reply code, err: %d\n", res);
        return res;
    }

    genlmsg_end(reply, msg_head);
    res = genlmsg_reply(reply, info);
    if (res != 0)
    {
        pr_err("Failed to send reply, error: %d", res);
        return res;
    }

    return 0;
}

int change_path_visibility(struct genl_info* info, bool hide)
{
    int res;
    char* path;
    enum COLLECTOR_COMMAND command;
    enum COLLECTOR_RESULT collector_res;

    res = mutex_lock_interruptible(&mtx);
    if (res != 0)
    {
        pr_err("Failed to get lock!\n");
        return res;
    }

    path = get_path(info);
    if (path)
    {
        collector_res = hide? hide_path(path) : unhide_path(path);
    }
    else
    {
        collector_res = COLLECTOR_ERROR_NO_PATH;
    }

    mutex_unlock(&mtx);

    command = hide? COLLECTOR_COMMAND_HIDE : COLLECTOR_COMMAND_UNHIDE;
    res = send_reply(info, command, collector_res);
    return res;
}

int hide_cb(struct sk_buff* sender_skb, struct genl_info* info)
{
    return change_path_visibility(info, true /* hide */);;
}

int unhide_cb(struct sk_buff* sender_skb, struct genl_info* info)
{
    return change_path_visibility(info, false /* hide */);;
}

int uninstall_cb(struct sk_buff* sender_skb, struct genl_info* info)
{
    int res;

    res = mutex_lock_interruptible(&mtx);
    if (res != 0)
    {
        pr_err("Failed to get lock!\n");
        return res;
    }

    res = send_reply(info, COLLECTOR_COMMAND_UNINSTALL, uninstall());
    mutex_unlock(&mtx);
    return res;
}

static int __init collector_module_init(void)
{
    int rc = genl_register_family(&family_descr);
    if (rc != 0)
    {
        pr_err("Failed to register family: %i\n", rc);
        return -1;
    }

    mutex_init(&mtx);
    pr_info("Module initialied");
    return 0;
}

static void __exit collector_module_exit(void)
{
    int res;
    mutex_destroy(&mtx);

    res = genl_unregister_family(&family_descr);
    if (res != 0)
    {
        pr_err("Failed to unregister family: %i\n", res);
    }

    pr_info("Module deinitialized");
}

module_init(collector_module_init);
module_exit(collector_module_exit);
