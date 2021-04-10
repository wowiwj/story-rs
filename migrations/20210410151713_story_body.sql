-- Add migration script here
CREATE TABLE if not exists `story_bodies`
(
    `id`         bigint unsigned   NOT NULL AUTO_INCREMENT,
    `story_id`   bigint unsigned   NOT NULL COMMENT '故事id',
    `body`       longtext COLLATE utf8mb4_unicode_ci COMMENT '故事内容',
    `b_index`    int unsigned      NOT NULL              DEFAULT '0' COMMENT '创建的位置',
    `is_draft`   tinyint(1)        NOT NULL              DEFAULT '1' COMMENT '是否为草稿',
    `dtype`      smallint unsigned NOT NULL COMMENT '内容的类型',
    `img_url`    varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '图片地址',
    `img_height` int unsigned      NOT NULL              DEFAULT '0' COMMENT '图片的高度',
    `img_width`  int unsigned      NOT NULL              DEFAULT '0' COMMENT '图片的宽度',
    `created_at` timestamp         not null              DEFAULT current_timestamp,
    `updated_at` timestamp         NULL                  DEFAULT NULL,
    `deleted_at` timestamp         NULL                  DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `idx_story_bodies_story_id` (`story_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4 comment = '故事详情表';
