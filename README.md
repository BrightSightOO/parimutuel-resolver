# Parimutuel Resolver

Program ID: `RS1njPGQsykXyyPGUiAC9dvPyoqw73vtMFPJhipibj1`

This program serves to bridge the [optimistic oracle][1] and the existing
Hedgehog Parimutuel markets.

[1]: https://github.com/BrightSightOO/oracle-be

## Developers

### Building

To build the program, run the command:

```
$ pnpm programs:build
```

The resulting binary will be in the `.bin` directory.

### Testing

To test the program, run the command:

```
$ pnpm programs:test
```

### Generating clients

To update the generated client code, run the command:

```
$ pnpm generate
```

### Local validator

To run a local test validator with the optimistic oracle and parimutuel
resolver programs, first build the programs, then run the command:

```
$ pnpm validator
```
