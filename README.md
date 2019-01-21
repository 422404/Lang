# Lang

## A generic™ language
Exemple of what it would looks like:
```
namespace Test.Hello;

use Lang;
use Lang.Hello.World;

.public
Person: Object {
    .private name: String;
    .private age: Int;

    .public
    .init
    init(name: String, age: Int): Person {
        super.init();
        this.name = name;
        this.age = age;
        return this;
    }

    .public
    .getter
    {
        // .public .getter code
        getName(): String {
            return this.name;
        }

        getAge(): Int {
            return this.age;
        }
    }

    .public
    isAdult(): Bool {
        return this.age >= 21;
    }
}

.public
Student: Person {
    .private
    ine: String;

    .public
    .init
    init(name: String, age: Int, ine: String): Student {
        super.init(name, age);
        this.ine = ine;
        return this;
    }

    .public
    .getter
    getIne(): String {
        return this.ine;
    }
}
```