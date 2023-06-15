export class Bag<T> {
  bag: T[] = [];
  constructor(private items: T[]) {}

  take() {
    if (this.bag.length === 0) this.refill();

    // acabamos de preencher caso esteja vazio
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    return this.bag.pop()!;
  }

  peek() {
    if (this.bag.length === 0) this.refill();

    // acabamos de preencher caso esteja vazio
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    return this.bag.at(-1)!;
  }

  private refill() {
    this.bag = [...this.items].sort(() => Math.random() - 0.5);
  }
}