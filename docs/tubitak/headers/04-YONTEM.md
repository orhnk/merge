# YÖNTEM

> Araştırma yönteminin, veri toplama araçlarının, deney ve gözlem düzeneklerinin
> ve verilerin analiz yönteminin verildiği bölümdür.

# Kullanılan Araçlar

`merge` emülatörünü geliştirmek için kullanılan temel yazılım dili Rust'tır.
Rust'ı tercih etmemizin arkasındaki sebepler aşağıda listelenmiştir.

## Cargo Paket Yöneticisi

Günümüz programlama dil paket yöneticilerinin modern standartlarına uygun bir
paket yöneticisi olan rust programlama dilinin `cargo` paket yöneticisi, `merge`
ekosisteminin geliştirilmesinde büyük rol oynamıştır.

`cargo`'yu seçmemizin temel nedeni modern program geliştirme araçları ile kolay
kullanılabilir bir yapılandırma sistemi ile gelmesiydi.

Rust'ın standart paket yöneticisi olan Cargo, `merge`'nin geliştirme, bakım,
sürdürülebilirlik ve dağıtım süreçlerde büyük kolaylık sağlamıştır. Başlıca
sistem programlama dilleri arasında sık karşılaşılan standart olmayan paket
yöneticilerine [C/C++ vcpkg canon] karşın Rust'ın standart olarak belirlediği
Cargo, diğer sistem programlama dilleri ile karşılaştırıldığında daha kolay
kullanılabilir bir yapılandırma sistemine sahiptir.

Proje geliştirme sürecinde cargo'nun kullandığı semantik versiyonlama (semver),
yanking gibi sistemlerin sağladığı sürdürülebilirlik (maintenance) kolaylığı ile
birlikte yerel derleme (local compilation) ile otomatik özellik yönetimi
(feature management), plugin yönetimi (plugin management) gibi özellikler, `merge`
projesini geliştirmeyi kolaylaştırmıştır.

## Yüksek Seviye Sözdizimi

Rust, C ve C++ gibi düşük seviye programlama dillerinin aksine yüksek seviye bir
sözdizimine sahiptir. Bu sayede programcılar, düşük seviye programlama
dillerinde karşılaştıkları okunabilirik, yeniden düzenleme (refactoring) gibi
konularda sıkıntılar yaşamazlar.

## Ödünç Alma Denetleyicisi (Borrow Checker)

Rust sahiplik (ownership) ve ödünç alma (borrowing) kavramları sayesinde bütün
bellek yönetimini derleme zamanında (compile-time) yapar. Bu sayede
programcılar, bellek yönetimi ile uğraşmak zorunda kalmazken aynı zamanda
işleyiş anındaki bellek hatalarının çoğunun önüne geçilmiş olur.
[bkz. reference cycles (referans döngüleri)](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

Rust programlama dilinin en önemli özelliklerinden biri de manuel bellek
yönetimi olmamasıdır. Rust, yazdığınız programı derlerken, programınızın bellek
yönetimini otomatik olarak yapar. Bu sayede programcılar, bellek yönetimi ile
uğraşmak zorunda kalmazlar. Bunun için yenilikçi bir yöntem olan
`borrow checker` sistemini kullanır. Bu sistem, programcıların yazdığı programın
risk oluşturmayacak biçimde olmasını zorunlu kılar. Bu kurallara uymayan
programlar derlenmez. Böylece tamamen soyutlanmış (abstracted) hızlı ve güvenli
programlar yazılabilir.

## Sistem Seviyesinde Performans

Rust, LLVM Derleyici altyapısını temel alan bir programlama dilidir. Bu sayede
sistem seviyesinde performans sağlar.

## Yeni Nesil Programlama

Rust yarı fonksiyonel bir programlama dili olduğundan [bkz.
data/figures/programming-languages-classification] fonksiyonel programlama
dillerinin sahip olduğu kısa ve okunabilir kod yapısına sahiptir.

Desteklediği güçlü makro sistemi projemizi geliştirirken kod tasarrufu yapmamızı
sağlamıştır.

Desen eşleme (pattern matching), Trait sistemi, Güçlü (Strongly-typed) ve
Cebirsel Veri Tipleri (Algebraic Data Types), fonksiyonel programlama dillerinde
öne çıkan ve programın geliştirilmesinde çarpım türleri (product types) yerine
toplam türleri (sum types) kullanarak daha temiz ve deyimsel (idiomatic) kod
yazılmasını sağlayan bir programlama paradigmasıdır.

Rust derleyicisi (rustc) Çapraz derleme (cross-compilation) sayesinde yazdığımız
programı farklı işletim sistemleri için derleyebilmesi, kullanım kolaylığı
sağlamıştır.

## Eskiye Uyumlu (Backward Compatiblity)

Rust programlama dili büyük ve genişleyen kominitesi ile genel programlama
dillerinin karşılaştığı bir problemle karşı karşıya kalmıştır. Örneğin Python2
ve Python3 arasındaki temel değişiklikler bile programlama forumları,
kütüphaneler gibi pek çok açıdan büyük ses getirmiştir. Rust bu sorunun
güvencesini resmi anlamda sunarak rust programlarının eski derleyiciler ile
kullanılabileceğini kesinleştirmiştir.

Bu sayede `merge`, eski veya yeni versiyon rust derleyicilerini deskteklemektedir.

# Genel Geliştirici Araçları

## Git Versiyon Kontrol Sistemi

Merge projesinin mekandan bağımsız ve eş zamanlı geliştirilebilmesi için bir
organizasyon sistemine ihtiyaç duyduk. Projeyi geliştirirken, sürdürülebilirlik,
geliştirme, test, dağıtım gibi pek çok aşamada işimizi kolaylaştırması adına
endüstriyel standartlardan birisi olan Git Versiyon Kontrol sistemini kullandık.

## Github

Sıkıntı takipçisi (issue tracker), kod incelemesi (code review), özellik
istekleri (feature request), wiki gibi geniş çaplı projeler için gerekli olan
yapıları oluşturacak ve Git ile entegre çalışacak bir barındırma servisi (host)
olarak GitHub platformunu kullandık.

Bu sayede `merge`'yi geliştirmek ve kullanıcıların karşılaştıkları problemleri
çözmek kolaylaşmıştır.

## JetBrains IDE, VSCode, Helix ve Vimacs

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
IDE'leri kusurlu çalışmaktadır. [Read-Only File System & absolute paths for
dependencies etc.] Bu nedenle bazı zamanlar VSCode kod editörü kullandık.

Linux kullanıcılarının sık kullandığı sistem kabuğu (system shell) terminalleri
içerisinde geçirdiğim zamanlar kendi neovim dağıtımım olan `vimacs`'i kullandım
[https://github.com/utfeight/vimacs]

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
prensiplerine uygun olarak temiz bir kod tabanı üzerine geliştirilmesi
planlanmıştır. Bunun için Endişelerin ayrılması ile doğru miktarda uyum ve
bağlantı gibi pek çok programlama prensibi göz önünde bulundurularak
tasarlanmıştır.

Geliştirilmesi için bir çok rust kütüphanesinden yararlanılmıştır. (bkz.
Cargo.toml's : TODO)

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
komutlarına çevirebilecektir.

## MgMIR

`merge`'in en inovatif yönlerinden birisi olan `MgMIR` kendisine benzeyen
sistemlerden ayrıldığı noktalardan birisidir. `merge`'in ileriye dönük
geliştirilmesi kolay olması adına konfigürasyon dosyalarını minimum hacimde
tutmaya çalıştık.

Veritabanı kullanımı açısından `merge` emülatörü, `mew` projesini andırmaktadır.
`merge`, aşağıda belirtildiği üzere `mew`'in eksik yanlarından ders alıp onları
geliştirmiştir.

### Minimal konfigürasyon hacimleri

LLVM derleyici altyapı sistemi ve JVM Byte Code gibi tasarım şekillerinden
esinlenerek `merge`, konfigürasyonu minimumda tutmak adına kendi MIR'ini
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
