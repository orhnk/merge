# Öneriler

> Bu bölümde benzer çalışmalar yapacak olanlara yol göstermesi bakımından öneriler varsa belirtilir.

## Versiyon Uyuşmazlıkları

Bir paket kayıt sistemine kaydedildikten sonra geliştirilen yeni versiyonlar ile kayıt sisteminin güncellenmesi gerekir.
Özellikle birden fazla paket kayıt sistemine kaydedilen yazılımların bütün sistemlerde güncellemesini yapmak zor
olabildiğinden bazı kayıt sistemleri yazılımın yeni versiyonlarına ait güncellemeleri alamaz. Sonuç olarak bir
sistemdeki versiyon ile diğerindeki versiyon birbirinden farklı olur ve her sistemde istenilen özellikler mevcut olmaz.

Bu problemi çözmek için bütün paket platformlarının birbirlerine bağlanması ve üst düzey bir yapılandırma sistemine
ihtiyaç vardır. Bu yapılandırma sistemi `Pacman`'in `PKGBUILD` kodları, `Nix`'in `derivation`'ları gibi pek çok farklı
standardı anlayabilecek kapasitede olmalıdır. Bunu bir GENE modülü olarak geliştirmek daha geniş kitlelere hitap etme
adına faydalı olacaktır.

## Yeni Bir Paket Yöneticisi

GENE programı geliştirilme aşamasındayken paket standardalizasyonu için belirlenmiş potansiyel iki çözüm yöntemi
belirlenmiştir:

1. Bütün paket sistemlerini anlamlandırabilen yeni bir paket yöneticisi
2. Bütün paket sistemlerini anlamlandırabilen ve sistem-tabanlı tercümanlık yapabilen yeni bir ekosistem

Bu çözümlerden ilki standartlaştırma yerine yeni bir standart ortaya attığından GENE problemin çözümü olarak
ikinci seçeneği kullanmaktadır.

Ancak fikirsel olarak bize katılmayan başka bir geliştirici, aşağıda belirtildiği üzere yeni bir sistem geliştirebilir.

### Bütün Paket Kayıt Sistemlerini Anlayan Yeni Bir Paket Yöneticisi

Bu çözüm her ne kadar GENE'nin problemi çözme yaklaşımına zıt olsa da. GENE yazılımını geliştirmeden önceki planlama
aşamasında potansiyel bir çözüm olarak belirlenmiştir. Temel çalışma mantığı, GENE'nin paket kayıt sistemlerinde isim
tekilleştirmesi yerine paket kayıt sistemlerinden indirebilmesidir. Bu açıdan var olan sistemlere adapte edilirken
oluşabilecek sıkıntılar ve genele hitap eden birstandardalizasyon yerine yeni bir standart ortaya koyduğundan GENE daha
fazla efor gerektirmesine rağmen isim standardalizasyonu yönünde belirlenmiştir.
