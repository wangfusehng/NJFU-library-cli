# NJFU-library-cli

NJFU-library-cli 是使用rust编写的实现图书馆登录,查询,预约,签到,签退,取消的命令行工具

![LICENSE](https://img.shields.io/badge/LICENSE-MIT-yellow)
![wakatime](https://wakatime.com/badge/user/cfee0eb2-658b-4917-a1ed-9801e76b961f/project/896c2bad-d07b-4cfd-bf71-35a4cb5d13dc.svg)

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)
![forthebadge](https://forthebadge.com/images/badges/built-with-love.svg)

## install

### for windows

- [click here to download](https://github.com/jyf-111/NJFU-library-cli/releases/)

- if you use scoop

```powershell
scoop install https://raw.githubusercontent.com/jyf-111/scoop-self/master/bucket/njfulib.json
# for chinese, if you are facing net work error, you can try pgproxy.com
scoop install https://ghproxy.com/https://raw.githubusercontent.com/jyf-111/scoop-self/master/bucket/njfulib.json
```

### for linux and macos

- [build from source](#build-from-source)

## how to use

### 查看帮助

```bash
njfulib --help
# or
njfulib -h
```

### 登录

```bash
njfulib login --username <username> --password <password>
# or
njfulib login -u <username> -p <password>
```

- 首次预约,取消,签退需要登录
- 查询不需要登录
- login 会把密码保存在`~/.njfu-library-cli.json`
- 可以通过`--username`和`--password`将账号密码传递给程序
- 也可以通过`NJFUUSERNAME`和`NJFUPASSWORD`将账号密码传递给程序

### 查询

```bash
# 按学生姓名查询
njfulib query --day=today --name <your name>
# or
njfulib query -n <your name>

# 按作为查询
njfulib query --site <site name>
# or
njfulib query -s <site name>
```

- --day:
可选的值为 `today` `tomorrow`, 默认为`today`

- `<site>`:
`njfulib info floor`可以查看所有支持的座位

### 查看所有预约

```bash
njfulib state
```

### 预约

```bash
# 预约到第一个成功的座位为止
njfulib reserve --day today [--sites <site>...] [--filter <floor>...] --start <start time> --end <end time>
# or
njfulib reserve [-s <site>...] [-f <floor>...] --start <start time> --end <end time> # --start --end 不可缩写
```

- --day:
可选的值为 `today` `tomorrow`, 默认为`today`

- -s,--site:
  - 后面可跟多个座位,预约从前往后,直到第一个预约成功
  - 若不指定site列表,则随机预约一个空座位

- -f --floor:

- `<floor>` :所有楼层
`njfulib info floor`查看详细信息

|floor|   name  |
| --- |   ---   |
|2F-A | 二层A区 |
|2F-B | 二层B区 |
|3F-A | 三层A区 |
|3F-B | 三层B区 |
|3F-C | 三层C区 |
|3FA- | 三楼夹层|
|4F-A | 四层A区 |
|4FA- | 四楼夹层|
|5F-A | 五层A区 |
|6F-A | 六层    |
|7F-A | 七层北侧|


### 取消

```bash
njfulib cancel <id>

```

- `<id>`: 使用`njfulib statue`获取未到期预约id

### 签到(not support yet)

```bash
njfulib in <id>
```

### 签退

```bash
njfulib out <id>
```

- `<id>`: 使用njfulib statue获取未到期预约id

### info

```bash
njfulib info <item>
```

- `<item>`: 可选的值 `floor` `author` `user`

## build from source

```bash
git clone https://github.com/jyf-111/NJFU-library-cli.git
cd NJFU-library-cli
cargo build
cargo run -- help
```

## roadmap

- 支持签到
- 支持空间预约
- 查询楼层座位分布图
