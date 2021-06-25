use std::{collections::HashMap, fmt};

/*

# Постановка задачи

Нужно реализовать структуру данных с интерфейсом insert, remove, get (как у
коллекции Map), а также добавить поддержку версионирования (вспомни git) со
следующим интерфейсом:

* Checkpoint - сохранить текущую версию;
* Rollback - откатить на определенную версию;
* Prune - забыть все версии кроме последней.

Нельзя использовать сторонние библиотеки, только std, запись на диск не
требуется

# Алгоритм

В качестве структуры данных из std можно использовать HashMap, чтобы иметь
возможность брать значение по определенному ключу.

Элемент будет представлять из себя структуру их HashMap и вектора для хранения
истории.
*/

// Новый тип для упрощения написания
type Map = HashMap<String, String>;

// Наша структура для хранения элементов и истории
struct Element {
    elem: Map,
    vec: Vec<Map>,
}

// Реализация Display для нашего элемента
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "element: {:#?}\n vector: {:#?}", self.elem, self.vec)
    }
}

// Необходимы для работы методы
impl Element {
    // Создание нового элемента
    #[allow(dead_code)]
    fn new() -> Element {
        Element {
            elem: (Map::new()),
            vec: (Vec::<Map>::new()),
        }
    }

    // Вставка новой пары элементы
    #[allow(dead_code)]
    fn insert(&mut self, key: String, value: String) {
        self.elem.insert(key, value);
    }

    // Удаление элемента по ключу
    #[allow(dead_code)]
    fn remove(&mut self, key: String) {
        self.elem.remove(&key);
    }

    // Взятие элемента по ключу, возвращаем Option, так как может не
    // существовать ключа
    #[allow(dead_code)]
    fn get(&self, key: String) -> Option<&String> {
        self.elem.get(&key)
    }

    // Взятие количества элементов в структуре
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.elem.len()
    }

    // Взятие числа элементов в истории
    #[allow(dead_code)]
    fn vec_len(&self) -> usize {
        self.vec.len()
    }

    // Сохранение текущего элемента в истории
    #[allow(dead_code)]
    fn checkpoint(&mut self) {
        self.vec.push(self.elem.clone());
    }

    // Восстановление элемента из истории по его номеру, начинаем с 1
    #[allow(dead_code)]
    fn rollback(&mut self, version: usize) {
        match self.vec.get(version - 1) {
            Some(v) => self.elem = v.clone(),
            None => (),
        };
    }

    // Очистка истории, оставляя последний элемент истории
    #[allow(dead_code)]
    fn prune(&mut self) {
        let e = self.vec.last().cloned();
        self.vec.clear();
        match e {
            Some(el) => self.vec.push(el.clone()),
            None => (),
        }
    }
}

fn main() {
    let mut element = Element::new();
    element.insert("key".to_string(), "value".to_string());
    println!("element: {}", element);
    element.get("key".to_string());
    println!("element: {}", element);
    element.remove("key".to_string());
    println!("element: {}, len: {}", element, element.len());
    element.checkpoint();
    element.rollback(0);
}

#[cfg(test)]
mod tests {
    use crate::Element;

    #[test]
    fn test_elem() {
        let element = Element::new();
        // element.insert("key".to_string(), "value".to_string());
        assert_eq!(element.get("key".to_string()), None);
    }

    #[test]
    fn test_insert_elem() {
        let mut element = Element::new();
        element.insert("key".to_string(), "value".to_string());
        assert_eq!(element.get("key".to_string()), Some(&"value".to_string()));
    }

    #[test]
    fn test_remove_elem() {
        let mut element = Element::new();
        element.insert("key1".to_string(), "value1".to_string());
        assert_eq!(element.get("key1".to_string()), Some(&"value1".to_string()));
        element.insert("key2".to_string(), "value2".to_string());
        assert_eq!(element.get("key2".to_string()), Some(&"value2".to_string()));
        element.insert("key3".to_string(), "value3".to_string());
        assert_eq!(element.get("key3".to_string()), Some(&"value3".to_string()));
        assert_eq!(element.len(), 3);
        element.remove("key1".to_string());
        assert_eq!(element.get("key1".to_string()), None);
    }

    #[test]
    fn test_checkpoint() {
        let mut element = Element::new();
        element.insert("key".to_string(), "value".to_string());
        assert_eq!(element.get("key".to_string()), Some(&"value".to_string()));
        element.checkpoint();
        assert_eq!(element.get("key".to_string()), Some(&"value".to_string()));
    }

    #[test]
    fn test_roolback() {
        let mut element = Element::new();
        element.insert("key".to_string(), "value".to_string());
        assert_eq!(element.get("key".to_string()), Some(&"value".to_string()));
        element.checkpoint();
        element.insert("key1".to_string(), "value1".to_string());
        assert_eq!(element.get("key".to_string()), Some(&"value".to_string()));
        assert_eq!(element.get("key1".to_string()), Some(&"value1".to_string()));
        element.checkpoint();
        element.rollback(1);
        assert_eq!(element.get("key1".to_string()), None);
    }

    #[test]
    fn test_prune() {
        let mut element = Element::new();
        element.insert("key".to_string(), "value".to_string());
        assert_eq!(element.get("key".to_string()), Some(&"value".to_string()));
        element.checkpoint();
        element.insert("key1".to_string(), "value1".to_string());
        assert_eq!(element.get("key".to_string()), Some(&"value".to_string()));
        assert_eq!(element.get("key1".to_string()), Some(&"value1".to_string()));
        element.checkpoint();
        element.rollback(1);
        assert_eq!(element.get("key1".to_string()), None);
        assert_eq!(element.vec_len(), 2);
        element.prune();
        assert_eq!(element.vec_len(), 1);
    }
}
