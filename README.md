# NJFU-library-cli

NJFU-library-cli 是使用rust编写的实现图书馆登录,查询,预约,签到,签退,取消的命令行工具

![LICENSE](https://img.shields.io/badge/LICENSE-MIT-yellow)
![wakatime](https://wakatime.com/badge/user/cfee0eb2-658b-4917-a1ed-9801e76b961f/project/896c2bad-d07b-4cfd-bf71-35a4cb5d13dc.svg)
[![auto-reserve](https://github.com/jyf-111/NJFU-library-cli/actions/workflows/auto-reserve.yml/badge.svg)](https://github.com/jyf-111/NJFU-library-cli/actions/workflows/auto-reserve.yml)

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)
![forthebadge](https://forthebadge.com/images/badges/built-with-love.svg)

## Features

- 登录
- 查询姓名和座位
- 预约座位或空间,随机预约座位
- 取消预约
- 签到
- 签退

结合github action `自动预约` `自动签到` `自动签退` 参考[wiki](https://github.com/jyf-111/NJFU-library-cli/wiki)

## Install

### for windows

- [click here to download](https://github.com/jyf-111/NJFU-library-cli/releases/)

- if you use scoop

```powershell
scoop bucket add jyf-scoop-self https://github.com/jyf-111/scoop-self
scoop update
scoop install njfulib
```

### for linux and macos

- see [Build from source](#Build-from-source)

## how to use

```bash
njfulib login -u <username> -p <password>
njfulib query -n <your name>
njfulib statue
njfulib reserve [-s <site>...] [-f <floor>...] --start <start time> --end <end time> -r 30
njfulib reserve -s <space>... -d 2 --start <start time> --end <end time> -u <user>...
njfulib cancel -i <id>
njfulib in -s <site>
njfulib out -i <id>
```

[具体参数解释](https://github.com/jyf-111/NJFU-library-cli/wiki/参数解释)

## Build from source

```bash
git clone https://github.com/jyf-111/NJFU-library-cli.git
cd NJFU-library-cli
cargo build --release
cd target/release
./njfulib -h
```

## Roadmap

- 查询楼层座位分布图
