; showcase.clj — demonstração das features implementadas

; --- tipos básicos ---
(def pi 3.14159)
(def msg "hello risp")
(def flag true)

; --- aritmética ---
(def circunferencia (* 2 pi 5))

; --- coleções literais ---
(def numeros '(1 2 3 4 5))
(def vetor [10 20 30 40 50])
(def conjunto #{1 2 3 3 2 1})   ; deduplicado → #{1 2 3}
(def pessoa {:nome "tamer" :idade 30 :ativo true})

; --- acesso a map por keyword ---
(def nome (:nome pessoa))

; --- quote: lista sem avaliação ---
(def simbolos '(a b c))

; --- funções ---
(defn quadrado [x]
  (* x x))

(defn soma-lista [lst]
  (reduce + 0 lst))

; --- let e do ---
(def resultado
  (let [a 3
        b 4]
    (do
      (def hipotenusa (quadrado (+ a b)))
      (* a b))))

; --- higher-order functions ---
(def quadrados (map quadrado [1 2 3 4 5]))
(def dobros    (map (fn [x] (* x 2)) numeros))
(def soma      (reduce + 0 vetor))

; --- apply ---
(def max-vetor (apply + vetor))

; --- conj ---
(def mais-numeros (conj vetor 60))
(def mais-set     (conj conjunto 4))

; --- sequências ---
(def primeiro (first numeros))
(def resto    (rest numeros))
(def segundo  (second numeros))
(def ultimo   (last vetor))
(def tamanho  (count vetor))
(def terceiro (nth vetor 2))

; --- saída ---
(println "=== showcase risp ===")
(println "pi:" pi)
(println "circunferência (r=5):" circunferencia)
(println "mensagem:" msg)
(println "nome da pessoa:" nome)
(println "conjunto deduplicado:" conjunto)
(println "símbolos quoted:" simbolos)
(println "soma-lista de numeros:" (soma-lista numeros))
(println "quadrados de 1..5:" quadrados)
(println "dobros de 1..5:" dobros)
(println "soma de vetor:" soma)
(println "apply + vetor:" max-vetor)
(println "primeiro:" primeiro)
(println "segundo:" segundo)
(println "último:" ultimo)
(println "tamanho:" tamanho)
(println "terceiro elemento:" terceiro)
(println "conj vetor 60:" mais-numeros)
(println "conj set 4:" mais-set)
(println "empty? []:" (empty? []))
(println "empty? vetor:" (empty? vetor))
