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
int hide(struct sk_buff *sender_skb, struct genl_info *info);
int unhide(struct sk_buff *sender_skb, struct genl_info *info);
int uninstall(struct sk_buff *sender_skb, struct genl_info *info);

struct genl_ops collector_ops[COLLECTOR_COMMAND_COUNT] =
{
    {
        .cmd = COLLECTOR_COMMAND_HIDE,
        .flags = 0,
        .internal_flags = 0,
        .doit = hide,
        .dumpit = NULL,
        .start = NULL,
        .done = NULL,
        .validate = 0
    },
    {
        .cmd = COLLECTOR_COMMAND_UNHIDE,
        .flags = 0,
        .internal_flags = 0,
        .doit = unhide,
        .dumpit = NULL,
        .start = NULL,
        .done = NULL,
        .validate = 0
    },
    {
        .cmd = COLLECTOR_COMMAND_UNINSTALL,
        .flags = 0,
        .internal_flags = 0,
        .doit = uninstall,
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

int hide_path(char* path){ return 0; }
int unhide_path(char* path){ return 0; }

enum COLLECTOR_RESULT get_path(struct genl_info* info, char** path)
{
    struct nlattr* attr;
    if (info == NULL)
    {
        pr_err("Info is null, aborting\n");
        return COLLECTOR_ERROR_PROTOCOL;
    }

    attr = info->attrs[COLLECTOR_ATTRIBUTE_MSG];
    if (!attr)
    {
        pr_err("Failed to get path, aborting\n");
        return COLLECTOR_ERROR_NO_PATH;
    }

    *path = (char*)nla_data(attr);
    if (*path == NULL)
    {
        pr_err("Error receiving path\n");
        return COLLECTOR_ERROR_NO_PATH;
    }

    pr_info("Received path: %s\n", *path);
    return COLLECTOR_OK;
}

enum COLLECTOR_RESULT send_reply(
    struct genl_info* info,
    enum COLLECTOR_COMMAND command,
    char* msg,
    struct sk_buff** reply)
{
    int res;
    void* msg_head;

    *reply = genlmsg_new(NLMSG_GOODSIZE, GFP_KERNEL);
    if (*reply == NULL)
    {
        pr_err("Failed to create reply skb\n");
        return COLLECTOR_ERROR_SYSTEM;
    }

    msg_head = genlmsg_put(*reply,
                         info->snd_portid,
                         info->snd_seq + 1,
                         &family_descr,
                         0, // flags
                         command);

    if (msg_head == NULL)
    {
        pr_err("Failed to create msg head\n");
        return COLLECTOR_ERROR_SYSTEM;
    }

    res = nla_put_string(*reply, COLLECTOR_ATTRIBUTE_MSG, msg);
    if (res != 0)
    {
        pr_err("Failed to put reply string, err: %d\n", res);
        return COLLECTOR_ERROR_SYSTEM;
    }

    genlmsg_end(*reply, msg_head);
    return COLLECTOR_OK;
}

int exec(struct sk_buff* sender_skb, struct genl_info* info, enum COLLECTOR_COMMAND command)
{
    int rc;
    char* path;
    struct sk_buff* reply;
    enum COLLECTOR_RESULT res;

    if (command != COLLECTOR_COMMAND_UNINSTALL)
    {
        res = get_path(info, &path);
        if (res != COLLECTOR_OK)
        {
            // TODO: Send response
            return -1;
        }
    }
    else
    {
        path = NULL;
    }

    // TODO: exec

    res = create_reply(info, command, path, &reply);
    if (res != COLLECTOR_OK)
        return -1;

    rc = genlmsg_reply(reply, info);
    if (rc != 0)
    {
        pr_err("Failed to send reply, error: %d", rc);
        return -1;
    }

    return 0;
}

int hide(struct sk_buff* sender_skb, struct genl_info* info)
{
    pr_info("%s() invoked\n", __func__);
    return exec(sender_skb, info, COLLECTOR_COMMAND_HIDE);
}

int unhide(struct sk_buff* sender_skb, struct genl_info* info)
{
    pr_info("%s() invoked\n", __func__);
    return exec(sender_skb, info, COLLECTOR_COMMAND_UNHIDE);
}

int uninstall(struct sk_buff* sender_skb, struct genl_info* info)
{
    pr_info("%s() invoked\n", __func__);
    return exec(sender_skb, info, COLLECTOR_COMMAND_UNINSTALL);
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

    /*ret = mutex_lock_interruptible(&dumpit_cb_progress_data.mtx);
    if (ret != 0) {
        pr_err("Failed to get lock!\n");
        return ret;
    }

    mutex_unlock(&dumpit_cb_progress_data.mtx);

    */
}

static void __exit collector_module_exit(void)
{
    mutex_destroy(&mtx);

    int ret = genl_unregister_family(&family_descr);
    if (ret != 0)
    {
        pr_err("Failed to unregister family: %i\n", ret);
    }

    pr_info("Module deinitialized");
}

module_init(collector_module_init);
module_exit(collector_module_exit);
