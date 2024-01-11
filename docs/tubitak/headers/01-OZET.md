# ÖZET

> Özetin tamamı 150-250 kelime arasında olmalıdır. Proje özetinde, çalışmanın
> ayrıntılarından, yorumlardan ve kaynaklardan bahsedilmez. Özette projenin
> amacı, kullanılan yöntem, yapılan gözlem ve elde edilen temel bulgular ve
> sonuçlardan birkaç cümle ile bahsedilir. Ayrıca proje özetinin altına, proje
> konusunu genel olarak yansıtan en fazla beş kelimeden oluşan anahtar kelimeler
> verilir. İdeal olan başlarken taslak bir özet oluşturup, çalışma bittiğinde
> proje raporunun içeriğine uygun bir şekilde özeti güncellemektir.

Son yıllarda dijital dünyada elde edilen yeni teknolojiler sayesinde program
geliştirmek için başvurulan yöntemlerin verimliliği artmıştır. Örneğin 70'li yılların yazılımcıları programlarını delikli kartlar üzerine
yazarken bu işlem daha sonra assmebly gibi düşük seviye makine dillerine,
ardından C gibi düşük seviye programlama dillerine, günümüzde de LLVM gibi
modern altyapılar kullanan Zig ve Rust benzeri programlama dillerine dönüşmüştür.

Yaşanan değişikliğin büyük bir kısmı da standardizasyon alanında gerçekleşmiştir. Mesela C++'ta standart bir paket yöneticisi bulunmaması
nedeniyle bağımlılık yönetimi aşamsında belirsizler yaşanırken Rust bunu standardize etmiştir.

Benzer sıkıntıları işletim sistemlerinin paket yöneticileri için de söyleyebiliriz.
Örnek olarak Arch Linux `pacman -S <paket>` komutu, Scoop paket yöneticisinde
`scoop install <paket>` şeklindedir. Sanal makine gibi çok sayıda işletim sistemi kullanımı gerektiren ortamlarda yazılımcılar yüzlerce komutla çalışmaktadır
Bu durum yaşanan kafa karışıklığının boyutunu ortaya koymaktadır.

Benzeri uyumsuzluklar, açık kaynaklı programları kurarken,
sanal makineler arası geçiş yaparken veya işletim sistemi değiştirirken
sıklıkla yaşanmaktadır.

Bu probleme çözüm olarak geliştirdiğimiz `merge` emülatörü,
herhangi bir sisteme bağlı kalmadan tanımlı bütün komutların kullanılabilmesine olanak sağlar.
Az önce verilen örnekten yola çıkacak olursak
Arch linux işletim sisteminde `pacman -S <paket>` komutunun yanında `scoop install <paket>` komutunu da kullanabilirsiniz.

Bu yöntem sayesinde az önce belirtilen sorunların tamamı çözülebilmektedir.

farklı sistemlerde aynı komutu anlamlandırabilmektedir.
`merge` kullanarak  aynı komutları kullanarak sistemle iletişim kurabilirsiniz.

> ANAHTAR KELIMELER: paket yöneticisi, işletim sistemi, standartlaştırma, çapraz
> platform, soyutlama

## Kaynaklar

- Punch Cards: <https://en.wikipedia.org/wiki/Punched_card_input/output>