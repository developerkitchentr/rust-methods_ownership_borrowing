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

- *Step3 3:* Ekrana yazma işlemini _report_ fonksiyonuna devrettik. bu ilk seferde bize moved 
hatası verdi. Bunu aşmanın ilk yöntemi olarak bu fonksiyondan değeri döndürmek ve kendine tekrar 
atamak oldu. Tahmin edeceğiniz gibi bu çok kötü bir yöntem.

