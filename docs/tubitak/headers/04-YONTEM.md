# YÖNTEM

> Araştırma yönteminin, veri toplama araçlarının, deney ve gözlem düzeneklerinin
> ve verilerin analiz yönteminin verildiği bölümdür.

`merge` emülatörünü etkili bir şekilde geliştirmek için kullandığımız araçlar
aşağıda listelenmiştir:

# Rust Programlama Dili

## Cargo Paket Yöneticisi

Günümüz programlama dili paket yöneticilerinin modern standartlarına uygun şekilde
yöneten `cargo`, `merge` ekosisteminin geliştirilmesinde büyük rol oynamıştır.

`cargo` sayesinde projemizi derlemek, test etmek, yayımlamak gibi eylemler
daha verimli hale getirilmiştir

## Yüksek Seviye Sözdizimi

C ve C++ gibi düşük seviye programlama dillerinin aksine Rust, yüksek seviye bir
sözdizimine sahiptir. Bu sayede programcılar, düşük seviye programlama
dillerinde karşılaştıkları okunabilirik, yeniden düzenleme (refactoring) gibi
konularda sıkıntılar yaşamazlar.

## Ödünç Alma Denetleyicisi (Borrow Checker)

Rust sahiplik (ownership) ve ödünç alma (borrowing) kavramları ile bütün
bellek yönetimini derleme zamanında yapar. Böylece
programcılar, bellek yönetimi ile uğraşmak zorunda kalmayıp
işleyiş anındaki bellek hatalarının çoğunun önüne geçmiş olur.

## Sistem Seviyesinde Performans

Rust, LLVM Derleyici altyapısını temel alan bir programlama dilidir. Bu yüzden
yüksek seviyesinde performans sağlar.

## Yeni Nesil Programlama

Rust yarı fonksiyonel bir programlama dili olduğundan dolayı fonksiyonel programlama
dillerinin sahip olduğu kısa ve okunabilir kod yapısına sahiptir.

Desteklediği güçlü makro sistemi projemizi geliştirirken kod tasarrufu yapmamızı
sağlamıştır.

Desen eşleme, trait sistemi, güçlü ve cebirsel veri tipleri, fonksiyonel programlama dillerinde
öne çıkan özellikler sayesinde temiz ve deyimsel (idiomatic) bir kod tabanı elde edilmiştir.

`rustc` ile birliklte çapraz derleyebildiğimiz `merge`,
bütün modern işletim sistemlerinde çalışabilmektedir.

## Eskiye Uyumluluk (Backward Compatiblity)

Rust eskiye uyumlu olduğu için `merge` kaynak kodunu ileri safhalardaki derleyiciler de
çalıştırabilecek ve böylece projemiz daha güvenli şekilde geliştirilebilecektir.

# Git Versiyon Kontrol Sistemi

Merge projesinin mekandan bağımsız ve eş zamanlı geliştirilebilmesi için bir
organizasyon sistemine ihtiyaç duyulmuştur. Proje geliştirilirken, sürdürülebilirlik,
geliştirme, test, dağıtım gibi pek çok aşamada kolaylık sağlanması adına
endüstriyel standartlardan birisi olan Git Versiyon Kontrol sistemini kullanıldı.

# Github

Sorun takipçisi (issue tracker), kod incelemesi (code review), özellik
talebi (feature request), wiki gibi geniş çaplı projeler için gerekli olan
yapıları oluşturacak ve Git ile entegre çalışacak bir barındırma servisi
olarak GitHub platformunu kullanılmıştır.

Bu sayede `merge`'in geliştirilmesi ve kullanıcıların karşılaştıkları problemlerin
daha hızlı çözülmesi hedeflenmiştir.

# JetBrains IDE, VSCode, Helix ve Vimacs

`merge`, IDE ve editör sektöründe profesyonel yazılımcılar tarafından önerilen JetBrains
temelli RustRover, CLion ve VSCode IDE'leri; Vimacs[1] ve Helix terminal editörleri
kullanılarak geliştirilmiştir.

Büyük bir kod tabanı ile çalışırken, kodun okunabilirliği ve yeniden
düzenlenebilirliği (refactoring) gibi konulara dikkat etilmesi gerektiğinden
kullanılan editörler, bulut kullanılarak senkorize edilmiş;
ileri düzeyde etkili araç entegrasyonları (Git, GitHub, DB, JetBrains AI, Github
Copilot vb.), güçlü grafiksel arayüz tasarımı, kod üretimi ve
düzenleme araçları, konfigüre edilip `merge` projesini geliştirmek
için kullanılımıştır.

4 farklı editörün aynı proje için kullanılmasının nedeni her bir editörün kendine
özgü güçlü yanlarının bulunmasıdır.

İşletim sistemi olarak Windows ve NixOS,
Linux sistem kabuğu olarak `nushell`, `bash` ve `fish`,
Sistem paket yöneticisi olarak `nix`,
Scriptler için `sh`,
Klavye konfigürasyon modu olarak `vim`,
Web temelli editor olarak replit ve rust-playground

kullanılmıştır.

Konfigurasyon dosyaları için bkz. Ek 2:

- NixOS işletim sistemi: <https://github.com/utfeight/dotnix>
- Vimacs:
  - Geliştirdiğim vimacs yazılımının kaynak kodu:
    <https://github.com/utfeight/vimacs>
  - Vimacs konfigurasyon dosyaları: <https://github.com/utfeight/vimax>
- JetBrains:
  - ideavimrc: <https://github.com/utfeight/dotideavimrc>
  - TODO: link dotnix ft
- VSCode:
  - Nix ile yazılmış dekleratif konfigürasyon: TODO: link dotnix ft
- bütün configurasyon dosyaları için: TODO: link em all

# Merge Algoritmaları

Merge, geliştirilmeye açık olarak tasarlanmak istenildiğinden temel programlama
prensiplerine uygun olarak temiz bir kod tabanı üzerine geliştirilmesi
planlanmıştır. Bunun için endişelerin ayrılması (Seperation of Concerns) ile doğru miktarda uyum ve
bağlantı (cohesion & coupling) gibi pek çok programlama prensibi göz önünde bulundurularak
tasarlanmıştır.

Geliştirilmesi için bir çok rust kütüphanesinden yararlanılmıştır. (bkz.
Cargo.toml's : TODO)

## MgTWIN

> Merge çift yönlü tercüme (`merge` Two-Way InterpretatioN)

Merge programının yeni bir standart oluşturmadan diğer standartları anlaması
için geliştirilmiş olan çift yönlü tercüman modülüdür. Bu modül, Paket
Yöneticilerinin eymleri için belirlediği komutları anlamlandırarak diğer paket
yöneticilerinin komutlarına çevirmek için geliştirilmiştir.

Örneğin dwm X pencere yöneticisini indirmek isteyen bir kullanıcı aşağıdaki
komutlardan herhangi birini kullanabilir.

```shell
merge apt install dwm
merge pacman -S dwm
merge emerge --install dwm
merge nix shell dwm
...
```

yukarıda verilen komutlar bütün sistemlerde çalışacaktır çünkü `merge`
bu komut sistemlerinin hepsini anlamlandırıp kullandığınız işletim sisteminin
komutlarına çevirebilmektedir.

## MgMIR

`merge`'in inovatif konfigürasyon stili için kullanılan `MgMIR`,
hacimsel olarak çok az yer kaplamasıyla kendisine benzeyen sistemlerden ayrılmaktadır. `merge`'in ileriye dönük
geliştirilmesinin kolay olması adına konfigürasyon dosyalarını minimum hacimde
tutulmaya yönelik bir ara dil üretilmiştir.

Veritabanı kullanımı açısından `merge` emülatörü, `mew` projesine benzemektedir.
`merge`, aşağıda belirtildiği üzere `mew`'in eksik yanlarından ders alıp onları
tamamlamıştır.

### Minimal konfigürasyon hacimleri

LLVM derleyici altyapı sistemi ve JVM Byte Code gibi tasarım şekillerinden
esinlenerek `merge`, konfigürasyonu minimumda tutmak adına kendi MIR'ini
geliştirdik. MgMIR adını verdiğimiz bu basit MIR, son kullanıcının `merge`'e
eklemeler yapmasını kolaylaştırmaktadır.

### Geniş Sistem Desteği

Mew yalnızca `pacman` ve `rpm` ile debian temelli paket yöneticilerine çevirim
yapabildiği için yeterli sayıda kullanıcıya hitap etmemektedir.

`merge` ise aşağıda listelenen bütün paket yöneticilerini desteklemektedir.

1. apk
2. apt
3. homebrew
4. choco
5. dnf
6. emerge
7. nix
8. pacman
9. scoop
10. yum
11. zypper

Bütün durumları aynanda anlamlandırabilen `merge` toplamda 121 farklı kombinasyon ile çalışabilir.

Yeni konfigürasyonlar eklemenin de kolay olduğu `merge`'in çalıştırabildiği komutların günden güne artması beklenmektedir.

## MgPMS (Merge Package Manager Search)

> Merge Paket arama aracı

`merge`'in yazdığınız komutları çalıştırabilmesi için sisteminiz hakkında bilgi
edinmesi gerekir. Bu verilere ulaşmak için geliştirdiğimiz `MgPMS` modülü,
sisteminizdeki paket yöneticilerini güvenli bir şekilde `merge`'e aktarır.

## MgCLI (Merge Commandline Interface)

> Merge Komut Satırı Arayüzü

Rust'ın güçlü presedürel makro sistemini kullanan `clap` kütüphanesini baz alan
`MgCLI` sayesinde `merge`'in kullanımı daha pratik hale getirilmiştir.

## MgDB (Merge Database)

> Merge Veri tabanı

`merge`, kendisine verdiğiniz komutları anlamlandırabilmek için MgDB adını
verdiğim bir hafıza birimi kullanır. Bu hafıza birimi varsayılan şekilde sistem
konfigürasyon dosyası olarak `$XDG_CONFIG_HOME` sistem değişkeninde bulunur.

Böylece `merge`'i gerçek zamanlı konfigüre edebilir ve istediğiniz mantıksal
terimleri `MgMIR` kullanarak tanımlayabilirsiniz.

# Ekler

GitHub: <https://en.wikipedia.org/wiki/GitHub> Rust as a functional lang:
<https://kerkour.com/rust-functional-programming>
