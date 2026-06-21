# Contributing to npltz

We are excited that you are interested in contributing to npltz!

Contributions of all kinds are welcome, whether you're fixing bugs, adding new features, or improving documentation. Follow this guide to get started.

## Before You Start

Before making any core changes or adding a feature, open an issue first to discuss the implementation there.

First, look through the codebase to understand the project structure and how the TUI interacts. This will help you contribute in a way that is consistent with the project design.

> [!WARNING]
> Please do not submit slop PRs if you don't know anything about coding or Rust. The current code is already more like slop, so please don't submit more slop here.
> I can usually understand what an AI slop PR looks like, and the PR description should be clear enough to reflect your changes.
> I am not saying you can't use AI, but any slop here will just make the codebase more trash.
> If you actually want to contribute to make the codebase better, and if you already have Rust experience, you are welcome to contribute here with any kind of changes.
> Just make sure the PR should not be total slop, and it should not look like you don't know what you are doing and are just pasting whatever AI gives you.
> But you can use AI to find bugs, trash code in the codebase, and improve things yourself without fully depending on AI. It is a good tool if you use it the right way.
> You should be able to explain what your PR actually does in the codebase, what you added, and why. All of that should be clearly provided in the PR description.

If you are introducing any core functionality, you must write unit tests for it.

## Requirements

You need to have Rust installed on your system, as npltz is completely written in Rust.

## Commit Message

Commits must be **GPG-signed** and **verified**. Unverified commits will not be accepted.

Follow the **Conventional Commits** style:

```
feat: your-commit-message
fix: your-commit-message
```

You can read more about it here:
https://gist.github.com/harilvfs/53cc86aa79ea4642356540aadc6bd87d

## Steps to Contribute

### 1. Fork the Repository

Click the **Fork** button in the top-right corner of the repository page to create your own copy.

### 2. Clone Your Fork

```sh
git clone https://github.com/your-username/npltz.git
cd npltz
```

### 3. Set the Original Repository as Upstream

```sh
git remote add upstream https://github.com/harilvfs/npltz.git
```

### 4. Create a New Branch

```sh
git checkout -b feature/your-branch-name
```

Examples of branch names:

- `feature/add-new-feature`
- `bugfix/fix-installation-issue`

### 5. Make Your Changes

Implement the necessary changes to the code or documentation.

Make sure your changes are clear, tested, and follow the project's coding style.

### 6. Run Checks Before Committing

```sh
cargo xtask ci
```

If your changes affect CLI output or docs, also run:

```sh
cargo xtask man-pages
```

If you are adding core functionality, make sure you also add unit tests.

### 7. Stage and Commit

```sh
git add .
git commit -m "feat: brief description of your changes"
```

### 8. Push to Your Fork

```sh
git push origin feature/your-branch-name
```

### 9. Submit a Pull Request

Go to the original [npltz repository](https://github.com/harilvfs/npltz) on GitHub.
Click **New Pull Request**, select your branch, and describe your changes clearly in detail.

## Syncing Your Fork

Keep your fork updated to avoid conflicts:

```sh
git fetch upstream
git merge upstream/main
```

## Guidelines

### Code Quality

- Follow the existing coding style of the project.
- Write clear, concise, and well-documented code.
- Keep changes consistent with the current project structure and TUI design.

### Testing

- Test your changes locally to make sure they work as expected.
- If your changes introduce new functionality, add the necessary tests where applicable.
- If your changes introduce core functionality, unit tests are required.

### Documentation

- If your changes involve new functionality, update the relevant documentation.
- Make sure the README and man pages reflect your changes where needed.

### Pull Request Notes

- Explain clearly what your PR changes.
- Explain why the change is needed.
- Mention any important implementation details if needed.
- If the PR fixes a bug, mention what the bug was and how your change fixes it.
