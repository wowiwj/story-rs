-- Add migration script here

CREATE TABLE if not exists stories
(
    `id`           bigint unsigned  NOT NULL AUTO_INCREMENT,
    `title`        varchar(100)     NOT NULL COMMENT '标题',
    `summary`      varchar(200)     NOT NULL DEFAULT '' COMMENT '简介',
    `cover_url`    varchar(255)              DEFAULT NULL COMMENT '封面图片',
    `state`        tinyint unsigned NOT NULL DEFAULT '0' COMMENT '状态',
    `is_secret`    tinyint(1)       NOT NULL DEFAULT '0' COMMENT '是否为秘密',
    `parent_id`    bigint unsigned  NOT NULL DEFAULT '0' COMMENT '父类id',
    `channel_id`   bigint unsigned  NOT NULL DEFAULT '0' COMMENT '频道id',
    `user_id`      bigint unsigned  NOT NULL DEFAULT '0' COMMENT '用户id',
    `desktop_url`  varchar(255)              DEFAULT NULL COMMENT '桌面地址',
    `published_at` datetime                  DEFAULT NULL COMMENT '发布时间',
    `created_at`   timestamp        NOT NULL DEFAULT current_timestamp,
    `updated_at`   timestamp        NULL     DEFAULT NULL,
    `deleted_at`   timestamp        NULL     DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `idx_stories_title` (`title`),
    KEY `idx_stories_parent_id` (`parent_id`),
    KEY `idx_stories_channel_id` (`channel_id`),
    KEY `idx_stories_user_id` (`user_id`),
    KEY `idx_stories_published_at` (`published_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4 comment = '故事表';
