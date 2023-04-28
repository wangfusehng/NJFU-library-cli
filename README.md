# NJFU-library-cli

南京林业大学图书馆cli

![rust](https://img.shields.io/badge/rust-1.68.2-green)
![LICENSE](https://img.shields.io/badge/LICENSE-MIT-yellow)
![wakatime](https://wakatime.com/badge/user/cfee0eb2-658b-4917-a1ed-9801e76b961f/project/896c2bad-d07b-4cfd-bf71-35a4cb5d13dc.svg)

NJFU-library-cli 是使用rust编写的实现图书馆登录,查询,预约,签到,签退,取消的命令行工具

## install

### for windows

- [click here to download](https://github.com/jyf-111/NJFU-library-cli/releases/download/v1.0.0/njfulib.exe)

- if you use scoop

```powershell
scoop install https://raw.githubusercontent.com/jyf-111/scoop-self/master/bucket/njfulib.json
# for chinese, if you are facing net work error, you can try pgproxy.com
scoop install https://ghproxy.com/https://raw.githubusercontent.com/jyf-111/scoop-self/master/bucket/njfulib.json
```

### for linux and macos

- [click here](#build-from-source)

## how to use

### 查看帮助

```bash
njfulib --help
# or
njfulib -h
```

### 登录

- 首次预约,取消,签退需要登录
- 查询不需要登录
- login 会把密码保存在`~/.njfu-library-cli.json`

```bash
njfulib login --username <username> --password <password>
# or
njfulib l -u <username> -p <password>
```

### 查询

```bash
# 按学生姓名查询
njfulib query --day=today --name <your name>
# or
njfulib q -n <your name> # 参数day默认为today

# 按作为查询
njfulib query --site <site name>
# or
njfulib q -s <site name>
```

所有楼层

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

### 查看所有预约

```bash
njfulib statue
# or
njfulib s
```

### 预约

```bash
njfulib reserve --day today --site <site> --start <start time> --end <end time>
# or
njfulib r -s <site> --start <start time> --end <end time> # --start --end 不可缩写
```

### 取消

```bash
njfulib cancel <id> # 使用njfulib statue获取未到期预约id
njfulib c <id>

```

### 签到(not support yet)

```bash
njfulib in <id>
# or
njfulib i <id>
```

### 签退

```bash
njfulib out <id> # 使用njfulib statue获取到期预约id
njfulib o <id>
```

### info

```bash
njfulib info floor
njfulib info author
```

## build from source

```bash
git clone https://github.com/jyf-111/NJFU-library-cli.git
cd NJFU-library-cli
cargo build
cargo run -- help
```

[here is doc](https://wycis.me/NJFU-library-cli/njfulib/)

## roadmap

- 支持签到
- 支持空间预约
- 查询楼层座位分布图
