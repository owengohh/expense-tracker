create extension if not exists pgcrypto;

create table if not exists app_user (
  id uuid primary key default gen_random_uuid(),
  email text unique not null,
  created_at timestamptz not null default now()
);

create table if not exists category (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references app_user(id) on delete cascade,
  name text not null,
  created_at timestamptz not null default now(),
  unique (user_id, name)
);

create table if not exists tag (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references app_user(id) on delete cascade,
  name text not null,
  created_at timestamptz not null default now(),
  unique (user_id, name)
);

create table if not exists transaction (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references app_user(id) on delete cascade,
  category_id uuid references category(id) on delete set null,
  title text not null,
  description text,
  amount numeric(12, 2) not null,
  currency char(3) not null,
  transaction_type text not null check (
    transaction_type in ('expense', 'income', 'transfer', 'refund')
  ),
  occurred_at timestamptz not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists transaction_tag (
  transaction_id uuid not null references transaction(id) on delete cascade,
  tag_id uuid not null references tag(id) on delete cascade,
  primary key (transaction_id, tag_id)
);

create table if not exists audit_log (
  id uuid primary key default gen_random_uuid(),
  occurred_at timestamptz not null default now(),
  actor text not null,
  action text not null,
  resource_type text not null,
  resource_id uuid not null,
  metadata jsonb not null default '{}'::jsonb
);

create index if not exists idx_transaction_user_occurred_at
  on transaction(user_id, occurred_at desc);

create index if not exists idx_transaction_category
  on transaction(category_id);

create index if not exists idx_transaction_tag_tag_id
  on transaction_tag(tag_id);

create index if not exists idx_audit_log_resource
  on audit_log(resource_type, resource_id);

create index if not exists idx_audit_log_occurred_at
  on audit_log(occurred_at desc);
