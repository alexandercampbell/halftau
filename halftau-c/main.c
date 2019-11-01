
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum elt_type {
	ELT_TYPE_INT,
	ELT_TYPE_DOUBLE,
	ELT_TYPE_BOOL,
	ELT_TYPE_STRING,
	ELT_TYPE_SYMBOL,
	ELT_TYPE_LIST,
	ELT_TYPE_FUNCTION,
	ELT_TYPE_BUILTIN,
	ELT_TYPE_MACRO,
	ELT_TYPE_NIL,
};

// Could use a union type to reduce the memory footprint of elt.
struct elt {
	enum elt_type type;

	// Used for ints and bools.
	// Bools are truthy iff prim_value_int != 0.
	int prim_value_int;

	// Used only for doubles.
	double prim_value_double;

	// If non-nil, this is guaranteed to be a list of symbols.
	// Used by ELT_TYPE_FUNCTION.
	struct elt *lexical_binds;

	// Used only for lists.
	struct elt *head, *tail;
};

struct elt *new_elt() {
	// FIXME: this leaks memory.
	// In the prototyping stage, this is acceptable.
	struct elt *e = malloc(sizeof(struct elt));
	memset(e, 0, sizeof(struct elt));
	e->type = ELT_TYPE_NIL;
	return e;
}

bool truthy(struct elt *e) {
	assert(e);
	if (e->type == ELT_TYPE_NIL) return false;
	if (e->type == ELT_TYPE_BOOL) return e->prim_value_int == 0;
	return true;
}

struct elt *eval(struct elt *e);

struct elt *eval_function(struct elt *fn, struct elt *args_raw) {
	assert(fn->type == ELT_TYPE_FUNCTION);
	assert(args_raw->type == ELT_TYPE_LIST);

	struct elt *args = new_elt();
	args->type = ELT_TYPE_LIST;
	args->head = args->tail = NULL;

	struct elt *p = args;
	while (args_raw->type != ELT_TYPE_NIL) {
		p->head = eval(args_raw->head);

		p->tail = new_elt();
		p->tail->type = ELT_TYPE_LIST;
		p = p->tail;

		args_raw = args_raw->tail;
		assert(args_raw->type == ELT_TYPE_LIST
			|| args_raw->type == ELT_TYPE_NIL);
	}
}

struct elt *eval(struct elt *e) {
	assert(e);

	switch (e->type) {
	case ELT_TYPE_INT:
	case ELT_TYPE_DOUBLE:
	case ELT_TYPE_BOOL:
	case ELT_TYPE_STRING:
	case ELT_TYPE_NIL:
		return e;

	case ELT_TYPE_LIST: {
		struct elt *function = eval(e->head);

		switch (function->type) {
		case ELT_TYPE_FUNCTION: {
			return eval_function(function, e->tail);
		}
		default:
			assert(false && "first elt of eval'd list is not a callable type");
		}
	}

	default: return NULL;
	}

}

int main(int argc, char *argv[]) {
	return 0;
}

