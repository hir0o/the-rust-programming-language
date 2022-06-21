use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.text");
    
    let f = match f {
        Ok(file) => file,
        // errorの種類を取得する
        // マッチガード
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.text") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("ファイル作れなかったお, {:?}", e);
                },
            }
        },
        Err(error) => {
            panic!("error!: {:?}", error);
        },
    }

    // Okなら値を返し、Errなら、panicする
    let f = File::oepn("hello.text").unwrap();
    // expectは、エラーメッセージを指定できる。
    let f = File::oepn("hello.text").expect("ファイルをひらけませんでした。");

}


// エラーの移譲
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.text");

    let f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ?演算子をつかて、上記と同じ挙動にする
fn read_username_from_file2() -> Result<String, io::Error> {
    let f = File::open("hello.text")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);
}

// メソッドチェーン？でさらに短く
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.text")?.read_to_string(&mut s)?;
    Ok(s);
}
