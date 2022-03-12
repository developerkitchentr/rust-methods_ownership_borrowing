# METHODS STRUCT OWNERSHIP SWAP
### Rust ile yazdığımız bir kodu adım adı refactoring yapıyoruz.

- *Step 1:* Öncelikle yazdığımız Hands ve Item struct'ları ile basit bir yapı oluşturuyoruz. 
Burada elde tutulan meyveleri simule edeceğiz. Lütfen kodun ilk halini dikkatlice izleyin.

- *Step 2:* Swap işlemi yapılıyor. Swap işlemi mutable değişkenler üzerinde yapılsa da rust 
bu durumda bile bize biw uyarı verir.
```
 help: try: `std::mem::swap(&mut hands.left, &mut hands.right)
```

std::mem::swap bize önerdiği metotdur. Buna alternatif olarak clippy'e swap işlemini
kabul etmesini belirtmek için şunu da main fonksiyonuna tanımlayabiliriz.

```
#[allow(clippy::manual_swap)]
```

- *Step 3:* Ekrana yazma işlemini _report_ fonksiyonuna devrettik. bu ilk seferde bize moved 
hatası verdi. Bunu aşmanın ilk yöntemi olarak bu fonksiyondan değeri döndürmek ve kendine tekrar 
atamak oldu. Tahmin edeceğiniz gibi bu çok kötü bir yöntem.

Daha iyi bir yöntem şu şekilde oldu:

``` rust
// borrowing
report(&hands);
```
**Borrowing** ile hands değişkenini report fonksiyonuna ödünç verdik ve değişken içinde
kullanıldıktan sonra sahipliği main fonksiyonuna geri döndü. (main->hands tam olarak)

- *Step 4:* Kodumuzu refactoring yapıyoruz yani biraz daha okunur hale getirme çabasındayız.
Ancak tam bu commit ile garip bir hata alıyoruz. _juggle_ fonksiyonunda işler karışıyor.
``` rust
fn juggle(hands: &mut Hands){                                                                                   │   35 ``` rust
     println!("Let's juggle");                                                                                   │   36
     let air = hands.left;                                                                                       │   37
     hands.left = hands.right;                                                                                   │   38 ```
     hands.right = air;                                                                                          │~
    }
```
Gelen değer _mut_ olarak borrow edilmiş ancak görünürde bir sorun olmamasına rağmen 
bize _mut_ bir değerin _move_ edilemeyeceğini belirtiyor. 

Bunu çözmek için bize en başta önerilen metodu kullanıyoruz.

``` rust
fn juggle(hands: &mut Hands){
    println!("Let's juggle");
    std::mem::swap(&mut hands.left, &mut hands.right)
}
```
Bu sefer oldu.

-*Step 5:* Hadi biraz Rust dilinden konuşalım. Evet bu commit ile yazdığımız her şeyi 
Rustca! yazdık. Rust dilinde Class yok ancak en az class yapısı kadar güçlü
**struct** ve **impl** yapısı var. Gerçekten mindset'i değiştirince çözülemeyecek
problem yok gibi.

```rust
pub fn new()-> Self {
            Hands {
                    left: Item {
                        what: "an apple".to_owned(),
                        present: true,
                    },
                    right: Item {
                        what: "an banana".to_owned(),
                        present: true,
                    },
            }
        }

```
Üstteki **new** metodunu bir class2ın contractor'ı gibi düşünebiliriz.
Biz öyle bir anlam yüklüyoruz kendine. Aslında yapısı itibari ile Dart dilindeki
**factory contractor** yapısına daha çok benziyor. Varsayılan struct nesnemisi oluşturuyoruz.

**->Self** geri dönüşü bize implement ettiğimiz struct ile aynı tipte bir dönüş yapacağımızı söylüyor.


```rust
        pub fn juggle(&mut self){
            println!("Let's juggle");
            std::mem::swap(&mut self.left, &mut self.right)
        }


```
Yukarıda **self** değerimizi neden mutable aldık. Çünkü içeride swap işlemi yapıyoruz ve 
nesnemizi mutable kullanmamız gerekiyor.



** _Lütfen commitleri takip ederek adım adım kod üzerinde ilerleyiniz._ **
** @Uygun Bodur **
