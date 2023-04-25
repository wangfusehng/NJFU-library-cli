# NJFU-library-cli

南京林业大学图书馆cli

![rust](https://img.shields.io/badge/rust-1.68.2-green)
![LICENSE](https://img.shields.io/badge/LICENSE-MIT-yellow)
![wakatime](https://wakatime.com/badge/user/cfee0eb2-658b-4917-a1ed-9801e76b961f/project/896c2bad-d07b-4cfd-bf71-35a4cb5d13dc.svg)

NJFU-library-cli 是使用rust编写的实现图书馆查询,预约,签到,签退的命令行工具

## how to use

- 查看帮助

```bash
njfulib --help
# or
njfulib -h
```

- 登录
  - 首次预约,取消,签退需要登录
  - 查询不需要登录
  - login 会把密码保存在`~/.njfu-library-cli.json`

```bash
njfulib login --username <username> --password <password>
# or
njfulib l -u <username> -p <password>
```

- 查询

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

```txt
2F-A
2F-B
3F-A
3F-B
3F-C
3FA- # 三楼夹层
4F-A
4FA- # 四楼夹层
5F-A
6F-A
7F-A
```

- 查看所有预约

```bash
njfulib statue
# or
njfulib s
```

- 取消

```bash
njfulib cancel <id> # 使用njfulib statue获取未到期预约id
njfulib c <id>

```

- 签到(not support yet)

```bash
njfulib in <id>
# or 
njfulib i <id>
```

- 签退

```bash
njfulib out <id> # 使用njfulib statue获取到期预约id
njfulib o <id>
```

## build from source

```bash
git clone https://github.com/jyf-111/NJFU-library-cli.git
cd NJFU-library-cli
cargo build
cargo run help
```

## roadmap

- 支持签到
