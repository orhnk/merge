# YÖNTEM

> Araştırma yönteminin, veri toplama araçlarının, deney ve gözlem düzeneklerinin
> ve verilerin analiz yönteminin verildiği bölümdür.

Projemizi geliştirirken kullandığımız araçları yazılımcılar arasında rabet
gören araçlardan seçmeye özen gösterdik.

Öncelikli olarak projemizi deyimsel Rust ile 4 farklı editör kullanarak gelişitirdik.

Yazılım dili olarak Rust'ı seçmemizin sebepleri aşağıda listelenmiştir:

## Cargo Paket Yöneticisi

Bakım, test, bağımlılık yönetimi, dökümantasyon üretimi ve paylaşımını
kolaylaştırmak için Rust'ın standart paket yöneticisi olan `cargo`'yu kullandık.

## Ödünç Alma Denetleyicisi (Borrow Checker)

Rust, sahiplik (ownership) ve ödünç alma (borrowing) kavramları sayesinde bütün
bellek yönetimini derleme zamanında yapar. Böylece programcılar, bellek yönetimi
ile uğraşmak zorunda kalmayıp, işleyiş anındaki bellek hatalarının büyük bir 
kısmının geçilmiş olur.

## Sistem Seviyesinde Performans

Rust, LLVM Derleyici altyapısını temel alan bir programlama dilidir. Bu nedenle
rakip dillerle karşılaştırıldığında yüksek performanslar sergilemektedir.

## Yeni Nesil Programlama

Rust yarı fonksiyonel bir programlama dili olduğundan fonksiyonel programlama
dillerinin sahip olduğu kısa ve okunabilir  kod yapısına sahiptir.

Desteklediği güçlü makro sistemi projemizi geliştirirken kod tasarrufu yapmamızı
sağlamıştır.

Desen eşleme, trait sistemi, Güçlü ve Cebirsel Veri Tipleri, fonksiyonel programlama dillerinde
öne çıkan ve programın geliştirilmesinde çarpım türleri (product types) yerine
toplam türleri (sum types) kullanarak daha temiz ve deyimsel (idiomatic) kod
yazılmasını sağladığından projemizi geliştirirken bize kolaylık sağlamıştır.

`rustc` çapraz derleme özelliği ile yazdığımız yazdığımız programlar 
bütün işletim sistemlerinde çalışabilir durumdadır.

## Eskiye Uyumlu (Backward Compatiblity)

Rust 2.0 planlanmadığından dolayı projemiz gelecekte çıkacak derleyicilerle de
kullanılabilecektir.

# Genel Geliştirici Araçları

## Git Versiyon Kontrol Sistemi

Merge projesinin mekandan bağımsız ve eş zamanlı geliştirilebilmek için bir
organizasyon sistemine ihtiyaç duyduk. Projeyi geliştirirken, sürdürülebilirlik,
geliştirme, test, dağıtım gibi pek çok aşamada işimizi kolaylaştırması adına
endüstriyel standartlardan birisi olan Git Versiyon Kontrol sistemini kullandık.

## Github

Sıkıntı takipçisi (issue tracker), kod incelemesi (code review), özellik
istekleri (feature request), wiki gibi geniş çaplı projeler için gerekli olan
yapıları oluşturacak ve Git ile entegre çalışacak bir barındırma servisi (host)
olarak GitHub platformunu kullandık.

## JetBrains, VSCode, Helix ve Vimacs

`merge`, IDE ve editör sektöründe profesyonel yazılımcıların önerdiği JetBrains
temelli RustRover, CLion IDE'leri, Neovim kod editörü, Rust ile yazılmış Helix
terminal editörü ve Microsoft tarafından geliştirilen VSCode uygulaması
kullanılarak geliştirilmiştir.

Büyük bir kod tabanı (code base) ile çalışırken, kodun okunabilirliği ve yeniden
düzenlenebilirliği (refactoring) gibi konulara dikkat etmek gerekir. Bu sebeple
JetBrains IDE'lerini her yerde kullanmamızı sağlayan bulut temelli auto-sync,
ileri düzeyde etkili araç entegrasyonları (Git, GitHub, DB, JetBrains AI, Github
Copilot vb.) güçlü grafiksel arayüz tasarımı (GUI), kod üretimi (codegen) ve
düzenleme (refactoring) araçları ile konfigüre edip `merge` projesini geliştirmek
için kullandık.

4 farklı editörü aynı proje için kullanmamızın sebebi her bir editörün kendine
özgü güçlü yanları olmasıdır.

Ana geliştirici makinesi Linux NixOS dağıtımı çalıştırdığından JetBrains
IDE'leri her zaman istediğimiz gibi çalışmamaktadır.
Bu nedenle bazı zamanlar VSCode kod editörü kullandık.

Linux kullanıcılarının sık kullandığı sistem kabuğu (system shell) terminalleri
içerisinde ise kendi neovim dağıtımım olan `vimacs`'i ve helix metin editörünü kullandım.
[https://github.com/utfeight/vimacs]

## İşletim Sistemi Ayrıntıları

Yazılımın büyük bir kısmı NixOS işletim sistemi ile dwm X pencere yönetcisi
kullanan bir ekosistemde geliştirilmiştir.

Dökümanların bir kısmı Windows üzerinde yazılmıştır.

Konfigurasyon dosyaları için bkz. Ek 2:

- NixOS işletim sistemi: https://github.com/utfeight/dotnix
- Vimacs:
  - Geliştirdiğim vimacs yazılımının kaynak kodu:
    https://github.com/utfeight/vimacs
  - Vimacs konfigurasyon dosyaları: https://github.com/utfeight/vimax
- JetBrains:
  - ideavimrc: https://github.com/utfeight/dotideavimrc
  - TODO: link dotnix ft
- VSCode:
  - Nix ile yazılmış dekleratif konfigürasyon: TODO: link dotnix ft
- bütün configurasyon dosyaları için: TODO: link em all

# Merge Algoritmaları

Merge, geliştirilmeye açık olarak tasarlanmak istenildiğinden temel programlama
prensiplerine uygun olarak temiz bir kod tabanı üzerine geliştirilmiştir. Bunun
için Endişelerin ayrılması ile doğru miktarda uyum ve bağlantı gibi pek çok
programlama prensibi göz önünde bulundurularak tasarlanmıştır.

Bağımlı olduğumuz için kütüphaneler için: [Ek III](https://github.com/denizbaba0/merge/blob/master/merge/Cargo.toml)

## MgTWIN

> Merge çift yönlü tercüme (`merge` Two-Way InterpretatioN)

Merge programının yeni bir standart oluşturmadan diğer standartları anlaması
için geliştirilmiş olan çift yönlü tercüman modülüdür. Bu modül, Paket
Yöneticilerinin eymleri için belirlediği komutları anlamlandırarak diğer paket
yöneticilerinin komutlarına çevirmek için geliştirilmiştir.

Örneğin DWM linux pencere yöneticisini indirmek isteyen bir kullanıcı aşağıdaki
komutlardan herhangi birini kullanabilir.

```shell
merge apt install dwm
merge pacman -S dwm
merge emerge --install dwm
merge nix shell dwm
...
```

yukarıda verilmiş olan bütün komutlar bütün sistemlerde çalışacaktır çünkü `merge`,
bu komut sistemlerinin hepsini anlamlandırıp kullandığınız işletim sisteminin
komutlarına çevirebilmektedir.

Böylece bütün kullanıcılar, hangi paket yöneticisini kullanmak istediklerini tercih edebilirler.

## MgMIR

`merge`'in en inovatif yönlerinden birisi olan `MgMIR`, kendisine benzeyen
sistemlerden ayrıldığı noktalardan birisidir. `merge`'in ileriye dönük
geliştirilmesi kolay olması adına konfigürasyon dosyalarını minimum hacimde
tutmaya çalıştık.

Veritabanı kullanımı açısından `merge` emülatörü, `mew` projesini andırmaktadır.
`merge`, aşağıda belirtildiği üzere `mew`'in eksik yanlarından ders alıp onları
geliştirmiştir.

### Minimal konfigürasyon hacimleri

LLVM derleyici altyapı sistemi ve JVM Byte Code gibi tasarım şekillerinden
esinlenerek `merge`, konfigürasyonu minimumda tutmak adına kendi MIR'imizi
geliştirdik. MgMIR adını verdiğimiz bu basit MIR, son kullanıcının `merge`'e
eklemeler yapmasını kolaylaştırmaktadır.

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

GitHub: https://en.wikipedia.org/wiki/GitHub Rust as a functional lang:
https://kerkour.com/rust-functional-programming
