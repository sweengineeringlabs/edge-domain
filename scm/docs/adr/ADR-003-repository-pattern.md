# ADR-003: Repository Pattern — domain contracts

**Status:** Accepted  
**Date:** 2026-06-12  
**Governing ADR:** [ADR-019](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-019-repository-pattern.md) — Repository Pattern

---

## Mandate

Define the repository persistence contracts for the `edge-domain` workspace: `Repository<T, Id>`, `QueryableRepository<T, Id>`, `RepositoryFactory`, `Spec<T>`.

---

## Sub-crate: `edge-domain-repository`

No intra-domain deps. Leaf contract crate — must not import any other `edge-domain-*` crate.

---

## Contracts

### `Repository<T, Id>`

```rust
pub trait Repository<T, Id>: Send + Sync
where
    T: Send + 'static,
    Id: Send + Sync + 'static,
{
    fn find<'a>(&'a self, id: &'a Id)
        -> BoxFuture<'a, Result<Option<T>, RepositoryError>>;

    fn save(&self, id: Id, entity: T)
        -> BoxFuture<'_, Result<(), RepositoryError>>;

    fn delete<'a>(&'a self, id: &'a Id)
        -> BoxFuture<'a, Result<bool, RepositoryError>>;

    fn list(&self)
        -> BoxFuture<'_, Result<Vec<T>, RepositoryError>>;

    // provided defaults:
    fn exists<'a>(&'a self, id: &'a Id)  -> BoxFuture<'a, Result<bool, RepositoryError>>;
    fn count(&self)                        -> BoxFuture<'_, Result<usize, RepositoryError>>;
    fn list_page(&self, offset: usize, limit: usize)
        -> BoxFuture<'_, Result<Page<T>, RepositoryError>>  where T: Clone;
}
```

`find` returns `Option<T>` — absence is not an error. `delete` returns `bool` — deleting a non-existent entity returns `false`, not an error.

### `QueryableRepository<T, Id>`

Extends `Repository<T, Id>` with `Spec`-based queries:

```rust
pub trait QueryableRepository<T, Id>: Repository<T, Id>
where
    T: Clone + Send + Sync + 'static,
    Id: Send + Sync + 'static,
{
    fn find_by<'a>(&'a self, spec: &'a dyn Spec<T>)
        -> BoxFuture<'a, Result<Vec<T>, RepositoryError>>;

    fn find_one_by<'a>(&'a self, spec: &'a dyn Spec<T>)
        -> BoxFuture<'a, Result<Option<T>, RepositoryError>>;

    fn count_by<'a>(&'a self, spec: &'a dyn Spec<T>)
        -> BoxFuture<'a, Result<usize, RepositoryError>>;
}
```

Default impls filter in-process. Database implementations SHOULD override with a pushed-down query.

### `Spec<T>`

```rust
pub trait Spec<T: Send + Sync>: Send + Sync {
    fn matches(&self, entity: &T) -> bool;
}
```

Domain predicates with no persistence leakage.

### `RepositoryFactory` (SAF)

```rust
pub trait RepositoryFactory {
    fn in_memory<T, Id>() -> InMemoryRepository<T, Id>
    where
        T: Clone + Send + Sync + 'static,
        Id: Hash + Eq + Clone + Send + Sync + 'static;
}
```

`InMemoryRepository` is the reference implementation for tests and noop wiring.

---

## SEA module layout

```
src/
├── api/
│   └── repository/
│       ├── traits/   ← Repository, QueryableRepository, RepositoryFactory, Spec
│       ├── types/    ← InMemoryRepository, Page
│       └── errors/   ← RepositoryError
├── core/             ← pub(crate) InMemoryRepository impl
├── saf/              ← re-exports RepositoryFactory
└── lib.rs
```

---

## Relationship to event sourcing

Projection read models (ADR-002) write into `Repository` instances. Event-sourced aggregate roots are NOT stored in a `Repository` — `EventStore` is their source of truth.

---

## Testing

- All unit tests use `RepositoryFactory::in_memory()`.
- Spec tests: construct a concrete `Spec<T>`, call `find_by`, assert filtered slice.
- `#[tokio::test]` for all async tests.
- Naming: `test_<action>_<condition>_<expectation>_happy/error/edge`.
