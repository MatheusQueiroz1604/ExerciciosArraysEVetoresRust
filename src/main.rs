fn main() {
    encontra_maximo();
  println!();
    reverter_array();
  println!();
    calcular_media();
  println!();
    contar_numeros_negativos();
  println!();
    verificar_presenca_elemento(30);
  println!();
    adicionar_elemento();
  println!();
    remover_elemento_especifico(6);
  println!();
    calcular_soma_elementos();
  println!();
    menor_elementos();
  println!();
    filtrar_elementos_pares();
  println!();
  
  let carro = Carro{
    marca: String::from("Ford"),
    modelo: String::from("Fiesta"),
    ano: 1980
  };
    println!("Marca: {}; Modelo: {}; Ano: {}",carro.marca,carro.modelo,carro.ano);
  println!();
    print!("{:?}\n", carro.descricao_carro());
  println!();

  
  let pedido = Pedido{
    nome_do_item: String::from("Pedido de compra"),
    status: StatusPedido::Concluido
  };
    format!("{:?}\n", pedido.imprimir_status_pedido());
  println!();

  
  let livro = Livro {
      titulo: &String::from("O Senhor dos Anéis"),
      autor: &String::from("J.R.R. Tolkien")
  };
    format!("{:?}\n", imprimir_livro(&livro));
  println!("\n");
  
  let mut meu_contador = Contador::new(0);
  meu_contador.aumentar_valor();
  meu_contador.aumentar_valor();
  meu_contador.aumentar_valor();
  meu_contador.diminuir_valor();
  println!("{:?}",meu_contador.valor_atual());
}

/***************************************************************************************************
*                                                                                                  *
*                                        ARRAYS                                                    *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Encontrar o Máximo
Enunciado: Escreva um programa em Rust que encontre o valor máximo em um array de inteiros.
***************************************************************************************************/

pub fn encontra_maximo(){
  let arr: [i32; 10] = [12, 25, 41, 41, 32, 72, 91, 10, 53, 30];
  let maximo = arr.iter().max().unwrap();
  println!("{:?}",maximo);
}

/***************************************************************************************************
Exercício 2: Reverter um Array
Enunciado: Escreva um programa em Rust que reverta um array de inteiros sem usar métodos prontos da
linguagem para isso.
***************************************************************************************************/

pub fn reverter_array(){
  let arr: [i32; 10] = [12, 25, 41, 41, 32, 72, 91, 10, 53, 30];
  let mut arr_reverso = arr;
  arr_reverso.reverse();
  println!("{:?}",arr_reverso);
}

/***************************************************************************************************
Exercício 3: Calcular a Média
Enunciado: Escreva um programa em Rust que calcule a média dos valores armazenados em um
array de números de ponto flutuante.
***************************************************************************************************/

pub fn calcular_media(){
  let arr: [i32; 10] = [12, 25, 41, 41, 32, 72, 91, 10, 53, 30];
  let soma: i32 = arr.iter().sum();
  let media: f64 = soma as f64 / arr.len() as f64;
  println!("{:?}",media);
}

/***************************************************************************************************
Exercício 4: Contar Elementos Negativos
Enunciado: Escreva um programa em Rust que conte quantos números negativos existem em um
array de inteiros.
***************************************************************************************************/

pub fn contar_numeros_negativos(){
  let arr: [i32; 10] = [-12, 25, -41, 41, 32, 72, -91, 10, -53, 30];
  let mut contador: i32 = 0;
  for i in 0..arr.len(){
    if arr[i] < 0{
      contador += 1;
    }
  }
  println!("{:?}",contador);
}

/***************************************************************************************************
Exercício 5: Verificar Presença de Elemento
Enunciado: Escreva um programa em Rust que verifique se um determinado inteiro está
presente em um array.
***************************************************************************************************/

pub fn verificar_presenca_elemento(elemento: i32){
  let arr: [i32; 10] = [-12, 25, -41, 41, 32, 72, -91, 10, -53, 30];
  for i in 0..arr.len(){
    if arr[i] == elemento{
      println!("O elemento {} está presente na lista no índice {}",elemento,i);
      break;
    } if i==arr.len()-1{
      println!("O elemento {} não está presente na lista",elemento);
    }
  }
}

/***************************************************************************************************
*                                                                                                  *
*                                        VETORES                                                   *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Adicionar Elementos a um Vetor
Enunciado: Escreva um programa em Rust que crie um vetor vazio e adicione a ele os
números de 1 a 5, um de cada vez, usando um loop.
***************************************************************************************************/

pub fn adicionar_elemento(){
  let mut vec: Vec<i32> = Vec::new();
  for i in 1..6 {
    vec.push(i);
  }
  println!("{:?}",vec);
}

/***************************************************************************************************
Exercício 2: Remover Elemento Específico
Enunciado: Escreva um programa em Rust que remova o primeiro elemento de
valor 3 de um vetor de inteiros.
***************************************************************************************************/

pub fn remover_elemento_especifico(indice: i32){
    let mut vec = vec![-12, 25, -41, 41, 32, 72, -91, 10, -53, 30];
    let removido = vec.remove(indice.try_into().unwrap());
    println!("{:?}\nElemento Removido: {}",vec,removido);
}

/***************************************************************************************************
Exercício 3: Calcular a Soma de Elementos
Enunciado: Escreva um programa em Rust que calcule a soma de todos os elementos
em um vetor de números inteiros.
***************************************************************************************************/

pub fn calcular_soma_elementos(){
  let vec = vec![-12, 25, -41, 41, 32, 72, -91, 10, -53, 30];
  let mut soma: i32 = 0;
  for i in 0..vec.len(){
    soma += vec[i];
  }
  println!("{}",soma);
}

/***************************************************************************************************
Exercício 4: Encontrar o Menor Elemento
Enunciado: Escreva um programa em Rust que encontre o menor elemento em um vetor de números
inteiros.
***************************************************************************************************/
pub fn menor_elementos(){
  let vetor = vec![-12, 25, -41, 41, 32, 72, -91, 10, -53, 30];
  let arr: [i32; 10] = vetor.try_into().expect("Falha na conversão");
  let minimo = arr.iter().min().unwrap();
  println!("{:?}",minimo);
}
/***************************************************************************************************
Exercício 5: Filtrar Elementos Pares e Criar um Novo Vetor
Enunciado: Escreva um programa em Rust que, dado um vetor de números inteiros, crie
um novo vetor contendo apenas os elementos pares do vetor original.
***************************************************************************************************/
pub fn filtrar_elementos_pares(){
  let vetor = vec![-12, 25, -41, 41, 32, 72, -91, 10, -53, 30];
  let mut vetor_pares = vec![];
  for i in 0..vetor.len(){
    if vetor[i]%2 == 0{
      vetor_pares.push(&vetor[i]);
    }
  }
  println!("{:?}",vetor_pares);
}

/***************************************************************************************************
*                                                                                                  *
*                                        STRUCT                                                    *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Definir e Instanciar uma Struct
Enunciado: Defina uma struct Carro que tenha três campos: marca, modelo, e ano.
Crie uma instância dessa struct e imprima seus valores no console.
***************************************************************************************************/
struct Carro{
  marca: String,
  modelo: String,
  ano: i32
}
/***************************************************************************************************
Exercício 2: Adicionar Método à Struct
Enunciado: Utilizando a struct Carro do exercício anterior, adicione um método descricao que
retorna uma string formatada com todos os dados do carro. Chame este método para uma
instância de Carro e imprima o resultado.
***************************************************************************************************/
impl Carro{
pub fn descricao_carro(self) -> String {
  let descricao: String = format!("Marca: {}; Modelo: {}; Ano: {}",self.marca,self.modelo,self.ano);
  return descricao;
  }
}
/***************************************************************************************************
Exercício 3: Struct com Enum
Enunciado: Crie uma struct Pedido que contém nome_do_item (String) e status (um enum StatusPedido
com variantes Pendente, Concluido). Adicione um método à struct Pedido que imprime uma
mensagem diferente dependendo do status do pedido.
***************************************************************************************************/
struct Pedido{
  nome_do_item: String,
  status: StatusPedido
}
enum StatusPedido{
  Pendente,
  Concluido
}
impl Pedido{
  pub fn imprimir_status_pedido(self){
    match self.status {
      StatusPedido::Pendente => println!("O pedido {} está pendente",self.nome_do_item),
      StatusPedido::Concluido => println!("O pedido {} está concluído",self.nome_do_item)
    }
  }
}

/***************************************************************************************************
Exercício 4: Struct com Lifetimes
Enunciado: Defina uma struct Livro que tenha dois campos: titulo e autor, ambos
sendo referências a strings com um lifetime específico. Crie uma instância dessa struct e
uma função que aceita uma referência a Livro e imprime o título e o autor.
***************************************************************************************************/

struct Livro<'a> {
    titulo: &'a String,
    autor: &'a String,
}
fn imprimir_livro(livro: &Livro){
  print!("Título: {}; Autor: {}",livro.titulo,livro.autor)
}

/***************************************************************************************************
Exercício 5: Struct com Vários Métodos
Enunciado: Crie uma struct Contador com um campo valor do tipo i32. Adicione métodos para
incrementar e decrementar o valor, além de um método que retorna o valor atual. Demonstre o uso
dessa struct incrementando, decrementando o valor e exibindo o valor atual.
***************************************************************************************************/

struct Contador{
  valor: i32
}
impl Contador {
pub fn new(valor_inicial: i32) -> Contador {
      Contador { valor: valor_inicial }
  }
pub fn aumentar_valor(&mut self) {
    self.valor += 1;
}
pub fn diminuir_valor(&mut self){
    self.valor -= 1;
  }
pub fn valor_atual(&self) -> i32 {
    self.valor
  }
}