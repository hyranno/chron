
export class Result<T, E> {
    constructor(public value: Ok<T> | Err<E>) {}
    ok() {return this.value instanceof Ok}
    unwrapOr(v: T): T {return (this.value instanceof Ok) ? this.value.value : v;}
    flatMap<T2>(f: (v: T)=>Result<T2, E>): Result<T2, E> {
        return (this.value instanceof Ok) ? f(this.value.value) : new Result<T2, E>(this.value);
    }
    map<T2>(f: (v: T)=>T2): Result<T2, E> {
        return this.flatMap((v) => new Result<T2, E>(new Ok(f(v))));
    }
}
abstract class ResultEnum<T> {
    constructor(public value: T) {}
}
export class Ok<T> extends ResultEnum<T> {
    [Symbol.toStringTag] = "Ok"
}
export class Err<T> extends ResultEnum<T> {
    [Symbol.toStringTag] = "Err"
}

