-- Add migration script here
create table if not exists `users`
(
    id                bigint unsigned auto_increment
        primary key,
    name              varchar(50)  not null,
    email             varchar(100) not null,
    phone             varchar(20)  null,
    gender            int unsigned           default 0 not null,
    email_verified_at timestamp    null,
    password          varchar(255) not null,
    created_at        timestamp    null null default current_timestamp,
    updated_at        timestamp    null,
    deleted_at        timestamp    null,
    key `idx_email` (`email`),
    key `idx_phone` (`phone`),
    key `idx_created_at` (`created_at`)
) engine = InnoDB
  default charset = utf8mb4 comment = '用户表';