> [!NOTE]
> Please prefer English language for all communication.

## Creating an issue

Before creating an issue please ensure that the problem is not [already reported](https://github.com/mvoof/kerb/issues).

## How to Contribute

1. **Install Git Hooks (Lefthook)**

   This project uses [Lefthook](https://github.com/evilmartians/lefthook) to enforce formatting, linting, and tests automatically on commit and push.

   Install Lefthook once (globally):

   ```sh
   # macOS / Linux
   brew install lefthook

   # Windows
   winget install Evilmartians.Lefthook
   ```

   Then activate the hooks in your local clone (run once after cloning):

   ```sh
   lefthook install
   ```

   After this, every `git commit` will automatically run `cargo fmt` and `cargo clippy`, and every `git push` will run `cargo test`. To verify the hooks are working:

   ```sh
   lefthook run pre-commit
   lefthook run pre-push
   ```

1. **Fork and Clone the Repository**

   First, create your own copy of the repository by clicking the "Fork" button on GitHub. Then, clone your fork to your local machine:

   ```sh
   git clone https://github.com/your-username/kerb.git
   cd kerb
   git remote add upstream https://github.com/mvoof/kerb.git
   ```

1. **Create a New Branch**

   ```sh
   git checkout -b feature/short-description
   ```

1. **Make Changes**
   Implement your feature or fix the bug. Be sure to follow the project's coding style, add tests, and add/adjust benchmarks if necessary.

1. **Verify and Format Your Code (Crucial!)**

   Before committing, ensure your code is clean, formatted, and fully functional. Please run the following checks locally:
   - **Formatting**: Make sure the code style conforms to standards. Run the Rust formatter:
     ```sh
     cargo fmt --all
     ```
   - **Linting**: Run Cargo's linter to make sure there are no warnings or suggestions:
     ```sh
     cargo clippy --all-targets --all-features
     ```
   - **Testing**: Verify that all integration and unit tests pass:
     ```sh
     cargo test --all-targets --all-features
     ```
   - **Benchmarking**: If your changes affect performance-critical components (such as string decoding, frame copies, or hot loops), run the benchmarks to verify there are no performance regressions:
     ```sh
     cargo bench --all-features
     ```

1. **Commit Changes**

   Once everything is verified and passes cleanly, commit your changes. We follow the **[Conventional Commits](https://www.conventionalcommits.org/)** specification for commit messages. This helps in keeping the repository history clean, understandable, and ready for automated changelog generation.

   Your commit message should follow this format:

   ```text
   <type>(<scope>): <description>
   ```

   - **`<scope>`** is optional but highly recommended (e.g., `iracing`, `ac`, `codegen`, `utils`, `deps`).
   - Common **`<type>`** prefixes:
     - `feat`: A new feature (e.g., `feat(iracing): add tyre wear telemetry variables`)
     - `fix`: A bug fix (e.g., `fix(ac): resolve crash during connection shutdown`)
     - `docs`: Documentation-only changes (e.g., `docs: update setup steps for LMU plugin`)
     - `style`: Code style changes (whitespace, formatting, semicolons) that do not affect execution logic
     - `refactor`: Code changes that neither fix a bug nor add a feature (e.g., restructuring modules)
     - `perf`: Changes that improve execution speed or resource usage (e.g., `perf(utils): optimize cp1252 decoder`)
     - `test`: Adding missing tests or correcting existing tests
     - `chore`: Build system changes, dependencies updates, or repository maintenance

   Example:

   ```sh
   git add .
   git commit -m "feat(iracing): implement async connection check"
   ```

1. **Keep Your Branch Up to Date**

   Before pushing, make sure your branch is rebased on top of the latest `main` to avoid merge conflicts and keep the history clean:

   ```sh
   git fetch upstream
   git rebase upstream/main
   ```

   If conflicts arise, resolve them, then continue:

   ```sh
   git rebase --continue
   ```

1. **Push Changes**

   ```sh
   git push -u origin feature/short-description
   ```

   If you had to rebase after already pushing, use `--force-with-lease`:

   ```sh
   git push --force-with-lease
   ```
