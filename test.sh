find modules -mindepth 1 -maxdepth 1 -type d | while read -r dir; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "Building $dir..."
    cd "$dir"
    
    # Get all features from Cargo.toml
    FEATURES=$(cargo metadata --format-version=1 | \
      jq -r '.packages[] | select(.manifest_path | endswith("Cargo.toml")) | .features | keys[]' 2>/dev/null || echo "")
    
    # If no features, build default
    if [ -z "$FEATURES" ]; then
      RUSTFLAGS="-C target-feature=+multivalue" cargo build --target wasm32-unknown-unknown --release
    else
      # Build each feature
      for feature in $FEATURES; do
        echo "Building with feature: $feature"
        RUSTFLAGS="-C target-feature=+multivalue" cargo build --target wasm32-unknown-unknown --release --features "$feature"
      done
    fi
    
    cd - > /dev/null
  fi
done
