fn main() {
	let a = A {
		b: B { c: C { d: 1 } },
	};

	let _d = a.b.c.d;

	let e = E(F(G(H(1))));

	let _n = e.0 .0 .0 .0;
}

struct A {
	b: B,
}

struct B {
	c: C,
}

struct C {
	d: i32,
}

struct E(F);

struct F(G);

struct G(H);

struct H(i32);
