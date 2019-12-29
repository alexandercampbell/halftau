
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

enum elt_type
{
	// alloc with calloc gives a null elt
	ELT_TYPE_NULL = 0,
	ELT_TYPE_INT,
	ELT_TYPE_DOUBLE,
	ELT_TYPE_BOOL,
	ELT_TYPE_STRING,
	ELT_TYPE_SYMBOL,
	ELT_TYPE_CONS,
	ELT_TYPE_FUNCTION,
	ELT_TYPE_BUILTIN,
	ELT_TYPE_MACRO,
};

// Could use a union type to reduce the memory footprint of elt.
struct elt
{
	enum elt_type type;

	// Used for ints and bools.
	// Bools are truthy iff prim_value_int != 0.
	int64_t prim_value_int;

	// Used only for doubles.
	// There's no float64_t in the standard,
	// so... fugg.
	double prim_value_double;

	// If non-null, this is guaranteed to be a list of symbols.
	// Used by ELT_TYPE_FUNCTION.
	struct elt *lexical_binds;

	// Used only for cons.
	struct elt *car, *cdr;
};

struct elt *new_elt()
{
	// FIXME: this leaks memory.
	// In the prototyping stage, this is acceptable.
	struct elt *e = calloc(1, sizeof(struct elt));
	e->type = ELT_TYPE_NULL;
	return e;
}

bool truthy(struct elt *e)
{
	assert(e);
	if (e->type == ELT_TYPE_NULL)
		return false;
	if (e->type == ELT_TYPE_BOOL)
		return e->prim_value_int == 0;
	return true;
}

struct elt *eval(struct elt *e);

struct elt *eval_function(struct elt *fn, struct elt *args_raw)
{
	assert(fn->type == ELT_TYPE_FUNCTION);
	assert(args_raw->type == ELT_TYPE_CONS);

	struct elt *args = new_elt();
	args->type = ELT_TYPE_CONS;

	struct elt *p = args;
	while (args_raw->type)
	{
		p->car = eval(args_raw->car);

		p->cdr = new_elt();
		p->cdr->type = ELT_TYPE_CONS;
		p = p->cdr;

		args_raw = args_raw->cdr;
		assert(args_raw->type == ELT_TYPE_CONS || args_raw->type == ELT_TYPE_NULL);
	}
}

struct elt *eval(struct elt *e)
{
	assert(e);

	switch (e->type)
	{
	case ELT_TYPE_INT:
	case ELT_TYPE_DOUBLE:
	case ELT_TYPE_BOOL:
	case ELT_TYPE_STRING:
	case ELT_TYPE_NULL:
		// value literals evaluate to themselves
		return e;

	case ELT_TYPE_CONS:
	{
		struct elt *function = eval(e->car);

		switch (function->type)
		{
		case ELT_TYPE_FUNCTION:
			return eval_function(function, e->cdr);

		default:
			assert(false && "first elt of eval'd list is not a callable type");
		}
	}

	default:
		return NULL;
	}
}

int main(int argc, char *argv[])
{
	return 0;
}
