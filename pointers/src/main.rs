use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    println!("=== CAPÍTULO 15: SMART POINTERS ===\n");

    // 1. Box<T> - Almacenamiento en el Heap
    println!("1. BOX<T> - ALMACENAMIENTO EN EL HEAP");
    let b = Box::new(5);
    println!("b = {}", b);

    // Lista enlazada con Box
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Lista enlazada: {:?}", list);
    println!();

    // 2. Deref Trait - Dereferenciación inteligente
    println!("2. DEREF TRAIT - DEREFERENCIACIÓN INTELIGENTE");

    struct MiBox<T>(T);

    impl<T> MiBox<T> {
        fn new(x: T) -> MiBox<T> {
            MiBox(x)
        }
    }

    impl<T> Deref for MiBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    fn saludar(nombre: &str) {
        println!("Hola, {}!", nombre);
    }

    let m = MiBox::new(String::from("Rust"));
    saludar(&m);
    println!();

    // 3. Drop Trait - Limpieza de recursos
    println!("3. DROP TRAIT - LIMPIEZA DE RECURSOS");

    struct RecursoPersonalizado {
        nombre: String,
    }

    impl Drop for RecursoPersonalizado {
        fn drop(&mut self) {
            println!(">>> Liberando recurso: {}", self.nombre);
        }
    }

    let _recurso1 = RecursoPersonalizado {
        nombre: String::from("Archivo 1"),
    };

    {
        let _recurso2 = RecursoPersonalizado {
            nombre: String::from("Archivo 2"),
        };
        println!("Dentro del scope interno");
    }
    println!("Fuera del scope interno");
    println!();

    // 4. Rc<T> - Conteo de referencias
    println!("4. RC<T> - CONTEO DE REFERENCIAS");

    #[derive(Debug)]
    enum ListaRc {
        Cons(i32, Rc<ListaRc>),
        Nil,
    }

    use ListaRc::{Cons as ConsRc, Nil as NilRc};

    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("Referencias a 'a': {}", Rc::strong_count(&a));

    let _b = ConsRc(3, Rc::clone(&a));
    println!("Referencias a 'a' después de b: {}", Rc::strong_count(&a));

    {
        let _c = ConsRc(4, Rc::clone(&a));
        println!("Referencias a 'a' después de c: {}", Rc::strong_count(&a));
    }

    println!(
        "Referencias a 'a' después que c sale del scope: {}",
        Rc::strong_count(&a)
    );
    println!();

    // 5. RefCell<T> - Mutabilidad interior
    println!("5. REFCELL<T> - MUTABILIDAD INTERIOR");

    #[derive(Debug)]
    struct Mensajero {
        mensajes: RefCell<Vec<String>>,
    }

    impl Mensajero {
        fn new() -> Mensajero {
            Mensajero {
                mensajes: RefCell::new(vec![]),
            }
        }

        fn enviar(&self, mensaje: &str) {
            self.mensajes.borrow_mut().push(String::from(mensaje));
        }
    }

    let mensajero = Mensajero::new();
    mensajero.enviar("Hola");
    mensajero.enviar("Mundo");
    println!("Mensajes: {:?}", mensajero.mensajes.borrow());
    println!();

    // 6. Combinando Rc y RefCell
    println!("6. COMBINANDO RC Y REFCELL");

    #[derive(Debug)]
    enum ListaCombinada {
        Cons(Rc<RefCell<i32>>, Rc<ListaCombinada>),
        Nil,
    }

    use ListaCombinada::{Cons as ConsComb, Nil as NilComb};

    let valor = Rc::new(RefCell::new(5));

    let a = Rc::new(ConsComb(Rc::clone(&valor), Rc::new(NilComb)));
    let _b = ConsComb(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let _c = ConsComb(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *valor.borrow_mut() += 10;
    println!("a después de modificar: {:?}", a);
    println!();

    // 7. Ciclos de referencias (DEMOSTRACIÓN - sin causar panic)
    println!("7. CICLOS DE REFERENCIAS (DEMOSTRACIÓN SEGURA)");

    #[derive(Debug)]
    enum ListaCiclo {
        Cons(i32, RefCell<Rc<ListaCiclo>>),
        Nil,
    }

    impl ListaCiclo {
        fn siguiente(&self) -> Option<&RefCell<Rc<ListaCiclo>>> {
            match self {
                ListaCiclo::Cons(_, item) => Some(item),
                ListaCiclo::Nil => None,
            }
        }
    }

    use ListaCiclo::{Cons as ConsCiclo, Nil as NilCiclo};

    let a_ciclo = Rc::new(ConsCiclo(5, RefCell::new(Rc::new(NilCiclo))));
    println!("Conteo inicial de a_ciclo: {}", Rc::strong_count(&a_ciclo));

    let b_ciclo = Rc::new(ConsCiclo(10, RefCell::new(Rc::clone(&a_ciclo))));
    println!(
        "Conteo de a_ciclo después de b_ciclo: {}",
        Rc::strong_count(&a_ciclo)
    );
    println!("Conteo de b_ciclo: {}", Rc::strong_count(&b_ciclo));

    // Solo mostramos la creación del ciclo sin ejecutarlo para evitar el stack overflow
    if let Some(enlace) = a_ciclo.siguiente() {
        *enlace.borrow_mut() = Rc::clone(&b_ciclo);
        println!("⚠️  Ciclo de referencia creado (no lo imprimimos para evitar stack overflow)");
    }

    println!("Conteo final de a_ciclo: {}", Rc::strong_count(&a_ciclo));
    println!("Conteo final de b_ciclo: {}", Rc::strong_count(&b_ciclo));
    println!();

    // 8. Weak<T> para evitar ciclos
    println!("8. WEAK<T> - EVITANDO CICLOS DE REFERENCIA");

    use std::rc::Weak;

    #[derive(Debug)]
    struct Nodo {
        valor: i32,
        padre: RefCell<Weak<Nodo>>,
        hijos: RefCell<Vec<Rc<Nodo>>>,
    }

    let hoja = Rc::new(Nodo {
        valor: 3,
        padre: RefCell::new(Weak::new()),
        hijos: RefCell::new(vec![]),
    });

    println!(
        "hoja fuerte = {}, débil = {}",
        Rc::strong_count(&hoja),
        Rc::weak_count(&hoja)
    );

    {
        let rama = Rc::new(Nodo {
            valor: 5,
            padre: RefCell::new(Weak::new()),
            hijos: RefCell::new(vec![Rc::clone(&hoja)]),
        });

        *hoja.padre.borrow_mut() = Rc::downgrade(&rama);

        println!(
            "rama fuerte = {}, débil = {}",
            Rc::strong_count(&rama),
            Rc::weak_count(&rama)
        );
        println!(
            "hoja fuerte = {}, débil = {}",
            Rc::strong_count(&hoja),
            Rc::weak_count(&hoja)
        );
    }

    println!("Después que rama sale del scope:");
    println!(
        "hoja fuerte = {}, débil = {}",
        Rc::strong_count(&hoja),
        Rc::weak_count(&hoja)
    );
    println!("Padre de hoja: {:?}", hoja.padre.borrow().upgrade());

    println!("\n=== FIN DEL CAPÍTULO 15 ===");
}

