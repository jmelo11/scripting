
cover-agent --source-file-path "src/nodes/evaluator.rs" --test-file-path "src/nodes/evaluator.rs" --code-coverage-report-path "coverage.xml" --test-command "cargo llvm-cov --output-path coverage.xml --covertura" --desired-coverage 95 --max-iterations 10 --additional-instructions "if using rust, put all new tests inside a mod called ai_gen_tests"
