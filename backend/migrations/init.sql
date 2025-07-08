create table if not exists public.failed_jobs
(
    id         serial
        primary key,
    connection text                                not null,
    queue      text                                not null,
    payload    text                                not null,
    exception  text                                not null,
    failed_at  timestamp default CURRENT_TIMESTAMP not null
);

alter table public.failed_jobs
    owner to purple;

create table if not exists public.purple_commission_log
(
    id             serial
        primary key,
    invite_user_id integer  not null,
    user_id        integer  not null,
    trade_no       char(36) not null,
    order_amount   integer  not null,
    get_amount     integer  not null,
    created_at     integer  not null,
    updated_at     integer  not null
);

alter table public.purple_commission_log
    owner to purple;

create table if not exists public.purple_coupon
(
    id                  serial
        primary key,
    code                varchar(255)          not null,
    name                varchar(255)          not null,
    type                boolean               not null,
    value               integer               not null,
    show                boolean default false not null,
    limit_use           integer,
    limit_use_with_user integer,
    limit_plan_ids      varchar(255),
    limit_period        varchar(255),
    started_at          integer               not null,
    ended_at            integer               not null,
    created_at          integer               not null,
    updated_at          integer               not null
);

alter table public.purple_coupon
    owner to purple;

create table if not exists public.purple_invite_code
(
    id         serial
        primary key,
    user_id    integer               not null,
    code       char(32)              not null,
    status     boolean default false not null,
    pv         integer default 0     not null,
    created_at integer               not null,
    updated_at integer               not null
);

alter table public.purple_invite_code
    owner to purple;

create table if not exists public.purple_knowledge
(
    id         serial
        primary key,
    language   char(5)               not null,
    category   varchar(255)          not null,
    title      varchar(255)          not null,
    body       text                  not null,
    sort       integer,
    show       boolean default false not null,
    created_at integer               not null,
    updated_at integer               not null
);

comment on table public.purple_knowledge is '知识库';

comment on column public.purple_knowledge.language is '語言';

comment on column public.purple_knowledge.category is '分類名';

comment on column public.purple_knowledge.title is '標題';

comment on column public.purple_knowledge.body is '內容';

comment on column public.purple_knowledge.sort is '排序';

comment on column public.purple_knowledge.show is '顯示';

comment on column public.purple_knowledge.created_at is '創建時間';

comment on column public.purple_knowledge.updated_at is '更新時間';

alter table public.purple_knowledge
    owner to purple;

create table if not exists public.purple_log
(
    id         serial
        primary key,
    title      text         not null,
    level      varchar(11),
    host       varchar(255),
    uri        varchar(255) not null,
    method     varchar(11)  not null,
    data       text,
    ip         varchar(128),
    context    text,
    created_at integer      not null,
    updated_at integer      not null
);

alter table public.purple_log
    owner to purple;

create table if not exists public.purple_mail_log
(
    id            serial
        primary key,
    email         varchar(64)  not null,
    subject       varchar(255) not null,
    template_name varchar(255) not null,
    error         text,
    created_at    integer      not null,
    updated_at    integer      not null
);

alter table public.purple_mail_log
    owner to purple;

create table if not exists public.purple_notice
(
    id         serial
        primary key,
    title      varchar(255)          not null,
    content    text                  not null,
    show       boolean default false not null,
    img_url    varchar(255),
    tags       varchar(255),
    created_at integer               not null,
    updated_at integer               not null
);

alter table public.purple_notice
    owner to purple;

create table if not exists public.purple_order
(
    id                        serial
        primary key,
    invite_user_id            integer,
    user_id                   integer               not null,
    plan_id                   integer               not null,
    coupon_id                 integer,
    payment_id                integer,
    type                      integer               not null,
    period                    varchar(255)          not null,
    trade_no                  varchar(36)           not null
        unique,
    callback_no               varchar(255),
    total_amount              integer               not null,
    handling_amount           integer,
    discount_amount           integer,
    surplus_amount            integer,
    refund_amount             integer,
    balance_amount            integer,
    surplus_order_ids         text,
    status                    boolean default false not null,
    commission_status         boolean default false not null,
    commission_balance        integer default 0     not null,
    actual_commission_balance integer,
    paid_at                   integer,
    created_at                integer               not null,
    updated_at                integer               not null
);

comment on column public.purple_order.type is '1新购2续费3升级';

comment on column public.purple_order.surplus_amount is '剩余价值';

comment on column public.purple_order.refund_amount is '退款金额';

comment on column public.purple_order.balance_amount is '使用余额';

comment on column public.purple_order.surplus_order_ids is '折抵订单';

comment on column public.purple_order.status is '0待支付1开通中2已取消3已完成4已折抵';

comment on column public.purple_order.commission_status is '0待确认1发放中2有效3无效';

comment on column public.purple_order.actual_commission_balance is '实际支付佣金';

alter table public.purple_order
    owner to purple;

create table if not exists public.purple_payment
(
    id                   serial
        primary key,
    uuid                 char(32)              not null,
    payment              varchar(16)           not null,
    name                 varchar(255)          not null,
    icon                 varchar(255),
    config               text                  not null,
    notify_domain        varchar(128),
    handling_fee_fixed   integer,
    handling_fee_percent numeric(5, 2),
    enable               boolean default false not null,
    sort                 integer,
    created_at           integer               not null,
    updated_at           integer               not null
);

alter table public.purple_payment
    owner to purple;

create table if not exists public.purple_plan
(
    id                   serial
        primary key,
    group_id             integer               not null,
    transfer_enable      integer               not null,
    name                 varchar(255)          not null,
    speed_limit          integer,
    show                 boolean default false not null,
    sort                 integer,
    renew                boolean default true  not null,
    content              text,
    month_price          integer,
    quarter_price        integer,
    half_year_price      integer,
    year_price           integer,
    two_year_price       integer,
    three_year_price     integer,
    onetime_price        integer,
    reset_price          integer,
    reset_traffic_method boolean,
    capacity_limit       integer,
    daily_unit_price     integer,
    transfer_unit_price  integer,
    created_at           integer               not null,
    updated_at           integer               not null
);

alter table public.purple_plan
    owner to purple;

create table if not exists public.purple_server_group
(
    id         serial
        primary key,
    name       varchar(255) not null,
    created_at integer      not null,
    updated_at integer      not null
);

alter table public.purple_server_group
    owner to purple;

create table if not exists public.purple_server_hysteria
(
    id                      serial
        primary key,
    group_id                varchar(255)          not null,
    route_id                varchar(255),
    name                    varchar(255)          not null,
    parent_id               integer,
    host                    varchar(255)          not null,
    port                    varchar(11)           not null,
    server_port             integer               not null,
    tags                    varchar(255),
    rate                    varchar(11)           not null,
    show                    boolean default false not null,
    sort                    integer,
    up_mbps                 integer               not null,
    down_mbps               integer               not null,
    server_name             varchar(64),
    insecure                boolean default false not null,
    ignore_client_bandwidth boolean default false not null,
    obfs_type               varchar(11),
    created_at              integer               not null,
    updated_at              integer               not null
);

alter table public.purple_server_hysteria
    owner to purple;

create table if not exists public.purple_server_route
(
    id           serial
        primary key,
    remarks      varchar(255) not null,
    match        text         not null,
    action       varchar(11)  not null,
    action_value varchar(255),
    created_at   integer      not null,
    updated_at   integer      not null
);

alter table public.purple_server_route
    owner to purple;

create table if not exists public.purple_server_shadowsocks
(
    id            serial
        primary key,
    group_id      varchar(255)          not null,
    route_id      varchar(255),
    parent_id     integer,
    tags          varchar(255),
    name          varchar(255)          not null,
    rate          varchar(11)           not null,
    host          varchar(255)          not null,
    port          varchar(11)           not null,
    server_port   integer               not null,
    cipher        varchar(255)          not null,
    obfs          char(11),
    obfs_settings varchar(255),
    show          boolean default false not null,
    sort          integer,
    created_at    integer               not null,
    updated_at    integer               not null
);

alter table public.purple_server_shadowsocks
    owner to purple;

create table if not exists public.purple_server_trojan
(
    id             serial
        primary key,
    group_id       varchar(255)          not null,
    route_id       varchar(255),
    parent_id      integer,
    tags           varchar(255),
    name           varchar(255)          not null,
    rate           varchar(11)           not null,
    host           varchar(255)          not null,
    port           varchar(11)           not null,
    server_port    integer               not null,
    allow_insecure boolean default false not null,
    server_name    varchar(255),
    show           boolean default false not null,
    sort           integer,
    created_at     integer               not null,
    updated_at     integer               not null
);

comment on table public.purple_server_trojan is 'trojan伺服器表';

comment on column public.purple_server_trojan.id is '节点ID';

comment on column public.purple_server_trojan.group_id is '节点组';

comment on column public.purple_server_trojan.parent_id is '父节点';

comment on column public.purple_server_trojan.tags is '节点标签';

comment on column public.purple_server_trojan.name is '节点名称';

comment on column public.purple_server_trojan.rate is '倍率';

comment on column public.purple_server_trojan.host is '主机名';

comment on column public.purple_server_trojan.port is '连接端口';

comment on column public.purple_server_trojan.server_port is '服务端口';

comment on column public.purple_server_trojan.allow_insecure is '是否允许不安全';

comment on column public.purple_server_trojan.show is '是否显示';

alter table public.purple_server_trojan
    owner to purple;

create table if not exists public.purple_server_vless
(
    id               serial
        primary key,
    group_id         text                  not null,
    route_id         text,
    name             varchar(255)          not null,
    parent_id        integer,
    host             varchar(255)          not null,
    port             integer               not null,
    server_port      integer               not null,
    tls              boolean               not null,
    tls_settings     text,
    flow             varchar(64),
    network          varchar(11)           not null,
    network_settings text,
    tags             text,
    rate             varchar(11)           not null,
    show             boolean default false not null,
    sort             integer,
    created_at       integer               not null,
    updated_at       integer               not null
);

alter table public.purple_server_vless
    owner to purple;

create table if not exists public.purple_server_vmess
(
    id              serial
        primary key,
    group_id        varchar(255)          not null,
    route_id        varchar(255),
    name            varchar(255)          not null,
    parent_id       integer,
    host            varchar(255)          not null,
    port            varchar(11)           not null,
    server_port     integer               not null,
    tls             boolean default false not null,
    tags            varchar(255),
    rate            varchar(11)           not null,
    network         varchar(11)           not null,
    rules           text,
    networksettings text,
    tlssettings     text,
    rulesettings    text,
    dnssettings     text,
    show            boolean default false not null,
    sort            integer,
    created_at      integer               not null,
    updated_at      integer               not null
);

alter table public.purple_server_vmess
    owner to purple;

create table if not exists public.purple_stat
(
    id                  serial
        primary key,
    record_at           integer     not null
        unique,
    record_type         char        not null,
    order_count         integer     not null,
    order_total         integer     not null,
    commission_count    integer     not null,
    commission_total    integer     not null,
    paid_count          integer     not null,
    paid_total          integer     not null,
    register_count      integer     not null,
    invite_count        integer     not null,
    transfer_used_total varchar(32) not null,
    created_at          integer     not null,
    updated_at          integer     not null
);

comment on table public.purple_stat is '订单统计';

comment on column public.purple_stat.order_count is '订单数量';

comment on column public.purple_stat.order_total is '订单合计';

comment on column public.purple_stat.commission_total is '佣金合计';

alter table public.purple_stat
    owner to purple;

create table if not exists public.purple_stat_server
(
    id          serial
        primary key,
    server_id   integer  not null,
    server_type char(11) not null,
    u           bigint   not null,
    d           bigint   not null,
    record_type char     not null,
    record_at   integer  not null,
    created_at  integer  not null,
    updated_at  integer  not null,
    unique (server_id, server_type, record_at)
);

comment on table public.purple_stat_server is '节点数据统计';

comment on column public.purple_stat_server.server_id is '节点id';

comment on column public.purple_stat_server.server_type is '节点类型';

comment on column public.purple_stat_server.record_type is 'd day m month';

comment on column public.purple_stat_server.record_at is '记录时间';

alter table public.purple_stat_server
    owner to purple;

create index if not exists idx_stat_server_record_at
    on public.purple_stat_server (record_at);

create index if not exists idx_stat_server_server_id
    on public.purple_stat_server (server_id);

create table if not exists public.purple_stat_user
(
    id          serial
        primary key,
    user_id     integer        not null,
    server_rate numeric(10, 2) not null,
    u           bigint         not null,
    d           bigint         not null,
    record_type char(2)        not null,
    record_at   integer        not null,
    created_at  integer        not null,
    updated_at  integer        not null,
    unique (server_rate, user_id, record_at)
);

alter table public.purple_stat_user
    owner to purple;

create index if not exists idx_stat_user_record_at
    on public.purple_stat_user (record_at);

create index if not exists idx_stat_user_server_rate
    on public.purple_stat_user (server_rate);

create index if not exists idx_stat_user_user_id
    on public.purple_stat_user (user_id);

create table if not exists public.purple_ticket
(
    id           serial
        primary key,
    user_id      integer               not null,
    subject      varchar(255)          not null,
    level        boolean               not null,
    status       boolean default false not null,
    reply_status boolean default true  not null,
    created_at   integer               not null,
    updated_at   integer               not null
);

comment on column public.purple_ticket.status is '0:已开启 1:已关闭';

comment on column public.purple_ticket.reply_status is '0:待回复 1:已回复';

alter table public.purple_ticket
    owner to purple;

create table if not exists public.purple_ticket_message
(
    id         serial
        primary key,
    user_id    integer not null,
    ticket_id  integer not null,
    message    text    not null,
    created_at integer not null,
    updated_at integer not null
);

alter table public.purple_ticket_message
    owner to purple;

create table if not exists public.purple_user
(
    id                 serial
        primary key,
    invite_user_id     integer,
    telegram_id        bigint,
    email              varchar(64)           not null
        unique,
    password           varchar(64)           not null,
    password_algo      char(10),
    password_salt      char(10),
    balance            integer default 0     not null,
    discount           integer,
    commission_type    boolean default false not null,
    commission_rate    integer,
    commission_balance integer default 0     not null,
    t                  integer default 0     not null,
    u                  bigint  default 0     not null,
    d                  bigint  default 0     not null,
    transfer_enable    bigint  default 0     not null,
    banned             boolean default false not null,
    is_admin           boolean default false not null,
    last_login_at      integer,
    is_staff           boolean default false not null,
    last_login_ip      integer,
    uuid               varchar(36)           not null,
    group_id           integer,
    plan_id            integer,
    speed_limit        integer,
    remind_expire      boolean default true,
    remind_traffic     boolean default true,
    token              char(32)              not null,
    expired_at         bigint  default 0,
    remarks            text,
    created_at         integer               not null,
    updated_at         integer               not null
);

comment on column public.purple_user.commission_type is '0: system 1: period 2: onetime';

alter table public.purple_user
    owner to purple;

