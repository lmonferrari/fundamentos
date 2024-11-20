use std::sync::{Arc, Mutex};

use std::thread;

fn main() {
    // criando um contador protegido por Mutex e compartilhado entre as threads usando Arc
    let counter = Arc::new(Mutex::new(0));

    // criando vetor de handles para as threads
    let mut handles = vec![];

    for _ in 0..10 {
        // clonando o Arc para compartilhar o contador entre as threads
        let counter = Arc::clone(&counter);

        // criando uma nova thread
        let handle = thread::spawn(move || {
            // criando um lock e acessando o contador
            let mut num = counter.lock().unwrap();

            // incrementando o contador
            *num += 1;
        });

        // adicionando o handle ao vetor
        handles.push(handle);
    }

    // esperando todas as threads finalizarem
    for handle in handles {
        handle.join().unwrap();
    }

    // imprimindo o valor final do contador
    print!("Contagem final: {}", counter.lock().unwrap());
}
