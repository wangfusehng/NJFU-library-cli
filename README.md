# NJFU-library-cli

NJFU-library-cli 是使用rust编写的实现图书馆登录,查询,预约,取消的命令行工具

![LICENSE](https://img.shields.io/badge/LICENSE-MIT-yellow)
![wakatime](https://wakatime.com/badge/user/cfee0eb2-658b-4917-a1ed-9801e76b961f/project/896c2bad-d07b-4cfd-bf71-35a4cb5d13dc.svg)
![Static Badge](https://img.shields.io/badge/language-rust-red)
![Static Badge](https://img.shields.io/badge/build_with-love-red)

## Features

- 登录
- 查询姓名和座位
- 预约座位或空间,随机预约座位
- 取消预约

结合github action `自动预约` 参考[wiki](https://github.com/jyf-111/NJFU-library-cli/wiki)

## Install

### Download

- [click here to download](https://github.com/jyf-111/NJFU-library-cli/releases/)

#### special for windows

- if you use scoop

```ps1
scoop bucket add jyf-scoop-self https://github.com/jyf-111/scoop-self
scoop update
scoop install njfulib
```

### Build from source

```bash
git clone https://github.com/jyf-111/NJFU-library-cli.git
cd NJFU-library-cli
cargo build --release
cd target/release
./njfulib -h
```

## how to use

```bash
njfulib login -u <username> -p <password> -c <cookie>
njfulib query -n <name>
njfulib query -n <site>
njfulib statue
njfulib reserve [-s <site>...] [-f <floor>...] --start <start time> --end <end time> -r 30
njfulib reserve -s <space>... -d 2 --start <start time> --end <end time> -u <user>...
njfulib cancel -u <uuid>
```

[具体参数解释](https://github.com/jyf-111/NJFU-library-cli/wiki/参数解释)
