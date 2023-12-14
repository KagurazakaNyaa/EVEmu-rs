# EVE Database Tool

Tool for init database

ref: <https://github.com/EvEmu-Project/EVEDBTool>

## Environment Variables

|Env|Default|Describe|
|---|---|---|
|DB_URL|mysql://evemu:evemu@db:3306/evemu|Database connection URL, must be compatible with [sqlx](https://github.com/launchbadge/sqlx)|
|SEED_MARKET|true|true/false|
|SEED_REGIONS|Derelik,The Citadel,The Forge|Define regions to be seeded|
|SEED_SATURATION|80|Set saturation level of seed|
