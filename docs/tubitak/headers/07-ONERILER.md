# Öneriler

> Bu bölümde benzer çalışmalar yapacak olanlara yol göstermesi bakımından öneriler varsa belirtilir.

## Kullanım Kolaylığının Önemi

`merge`'i geliştirmeye başladığımız zaman kompleks ayırıcı algoritmalar ve gerçekleştirilmesi zor planlar içerisindeydik.
Daha sonraları projemizi sadeleştirmenin hem bakım faaliyetleri hem de sonuç bakımından bize daha faydalı olacağını düşündük
ve kod tabanımızı sadeleştirdik.

Böylece kullanımı ve geliştirilmesi kolay bir kod tabanı elde ettik.

## Veri Tabanı Gerekli mi?

Veri tabanı kod tabanımızı basitleştirmekte çok yardımcı olduğundan dolayı
programınızı yazarken sizlere rahatlık sağlayacaktır.

## Veri Tabanı Derlenen Koda Dahil Olmalı Mı?

`merge` veritabanını konfigüre edilebilir tutma amacı ile verilerini sistem dosyalarınızdan çekmektedir.
Eğer daha az esnek bir yapıya sahip bir proje ile yalnızca kullanımı kolaylaştırmak istiyorsanız
veritabanınızı programınıza gömmeyi tercih edebilirsiniz.

## Çözümleme (Parsing) Gerekli Mi?

`merge` `MgMIR` dışında cüzi miktarda çözümleme yapmakla beraber kompleks algoritmalar kullanmamaktadır.
Komut satırından edindiğiniz kod parçaları yapıları bakımından basit oldukları için regex gibi yazı
manipülasyon sistemleri kullanılabilir.
