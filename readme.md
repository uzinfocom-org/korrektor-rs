<header>
<img src="https://raw.githubusercontent.com/uzinfocom-org/website/main/src/images/logo.svg" alt="logo" height="100" align="left">
<h1 style="display: inline">Korrektor Rust Lib</h1>

Dasturlar endilikda O'zbek tilini ham tushunadi!

[![GitHub top language](https://img.shields.io/github/languages/top/uzinfocom-org/korrektor-rs?style=flat-square&logo=github)](https://github.com/uzinfocom-org/vicardi)
[![Chat](https://img.shields.io/badge/Chat-grey?style=flat-square&logo=telegram)](https://t.me/korrektuz)
[![Test CI](https://github.com/uzinfocom-org/korrektor-rs/actions/workflows/test.yml/badge.svg)](https://github.com/uzinfocom-org/korrektor-rs/actions/workflows/test.yml)
</header>

## Kutubxona haqida

Ushbu kutubxona Rust dasturlash tilida yaratilgan bo'lib O'zbek tili bilan bog'liq turli matn manipulatsiyalarini amalga 
oshirishga yordam beradi. Shuningdek kutubxona WASM formatlarini ham o'z ichiga oladi, bu orqali siz kutubxonani yuqori 
dasturlash tillariga ham hech qanday muammolarsiz integratsiya qilishingiz mumkin.

## Qulayliklar

- Ma'lumotlarni o'zbek alifbosi tartibida saralash
- Tokenizatsiya. O'zbek tili imlo qoidalariga asosan so'zlarni bo'ginlarga ajratish
- Matndagi so'zlar chastotasini hisoblash
- Dublikatlar tozalash
- _Yanada ko'proq imkoniyatlar keyingi relizlarda..._

> Bu loyiha hozir sinov bosqichidan o'tmoqda. Agarda biror xatolikka duchor
> bo'lsangiz, xatolik haqida [xabardor](https://github.com/uzinfocom-org/korrektor-rs/issues/new)
> qilishni unutmang.

## O'rnatish

Quyidagi konfiguratsion qismni Cargo.toml fayliga joylashtiring:

```toml
[dependencies]
korrektor = "0.1.0"
```

## Litsenziya

Ushbu kutubxona MIT va Apache-2 ikki litsenziyasi ostida tarqatiladi. Batafsil ma'lumot uchun [LICENSE-MIT](./license-mit) va [LICENSE-APACHE](./license-apache) fayllarini ko'zdan kechiring!
