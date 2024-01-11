# GİRİŞ

> Giriş, araştırma konusu hakkında yapılmış araştırmaların sonuçlarının ve bu alanda cevapsız olan soruların bilimsel
> makalelere dayandırılarak anlatıldığı (kaynak taraması) bölümdür.

[//]: # (GENE, projesi yapılan derin literatür araştırması sonucunda benzersiz bir proje olarak ortaya çıkmıştır.)

Paket yöneticilerinin belirli bir standarda bağlı olmaması problemi şimdiye kadar bazı
yazılımlar tarafından giderilmeye çalışılsa da yeterli düzeyde standartlaşma elde edilememiştir.

Yapılan literatür araştırması sonucu `merge` ile benzerlik gösteren projeler aşağıda listelenmiştir.

## Pacapt

Pacapt, adından anlaşılacağı üzere bütün paket yöneticisi komutlarını `pacman` paket yöneticisine
benzetmeye çalışmaktadır.

POSIX SH ile yazılan Pacapt, daha önceden `pacman` kullanmamış olan yazılımcılara hitap
etmemektedir.

## Mew

Mew paket yönetici komutlarını standartlaştırma konusunda kullanıcılara yardımcı olmayı hedefleyen ufak çaplı bir
projedir. Mew, `.PO` ([GNU gettext utilities](https://www.gnu.org/software/gettext/manual/html_node/PO-Files.html))
dosyaları gibi çalışır.  Geliştiricisi, 6 yıl önce projeyi geliştirmeyi bırakmıştır.

Planlarının çoğu tamamlanmadan bırakılmış olan Mew, veri tabanında yalnızca `rpm` ve `pacman`'i
debian paket yöneticilerine çevirebilmektedir.

Merge ise sadece bir kaç paket yöneticisiyle sınırlı olmayıp sık kullanılan bütün sistemler arasında
çapraz çevirme yapabilmektedir.

## Pacman Rosetta

- Site: <https://wiki.archlinux.org/title/Pacman/Rosetta>

İnternet üzerinde, paket yönetici komutlarının eşleştirildiği bir dökümandır. Dökümanda yer alan paket yöneticisi sayıları kısıtlı olduğundan
ve yazılımcının dökümanı okuması gerektiğinden yeterli değildir.

Sonuç olarak sistem komut standartlaştırması için yeterli yazılım bulunmamaktadır.
Bu nedenle `merge` projesinin alanında etkili olması öngörülmektedir.

## Kaynaklar

- yumitude: <https://github.com/timols/yumitude> Project with the same idea but not implemented.
- MEW: <https://github.com/fossasia/mew> Project similar to pacaptr.
- whohas: <https://github.com/whohas/whohas> - A system utility to search from general package registries.
- pacaptr: <https://github.com/icy/pacapt> - pacman-like syntax for all package managers.
- bedrock linux: <https://bedrocklinux.org/0.7/pmm-beta.html> package manager manager (a.k.a pmm)
