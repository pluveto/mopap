# MOPAP

MOPAP (Multilingual Open Personal Accounting Protocal) is a simple, open, and human-readable format for personal accounting data. It is designed to be easy to read and write by humans, and easy to parse by computers. It supports multiple languages.

MOPAP（多语言开放个人记账协议）是一种简单、开放、人类可读的个人会计数据格式。它旨在方便人类阅读和编写，同时也方便计算机解析。支持多种语言的关键字。

At the same time, it is compatible with the [Markdown](https://daringfireball.net/projects/markdown/) format, which is widely used in the world. So you can edit it with any Markdown editor.

## Examples

### English

```md
# Account

## Basic Info

	name: default
	currency: USD

## Initial State

	BOA		balance 1958.52 USD

# Records

## 1999-01-01

	-13.20   USD    BOA    buy plain t-shirts #clothes
	+1000.00 USD    BOA    salary #salary

## 1999-01-02

	-9.99    USD    BOA    buy a book #books
	-0.50    USD    BOA    buy a cup of coffee #food
	-150.23  USD    BOA    a new phone #electronics
```

### Chinese

```md
# 账户

## 基本信息

	名称：默认
	货币：CNY

## 初始状态

	ICBC		余额 12000.00 CNY

# 记录

## 2021-07-01

	-23.00   CNY    ICBC    购买早餐 #饮食
	-32.50   CNY    ICBC    购买午餐 #饮食
	+5000.00 CNY    ICBC    工资 #工资

## 2021-07-02

	-15.00   CNY    ICBC    购买晚餐 #饮食
	-3.00    CNY    ICBC    购买零食 #零食
	-300.00  CNY    ICBC    购买新手机 #电子产品
```

## Parser

We have written a parser for MOPAP in Rust. You can use it to parse MOPAP files into JSON or any other format.
