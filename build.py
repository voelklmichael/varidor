
from shutil import copyfile
from subprocess import call

#call('cargo build --target wasm32-unknown-unknown --release')
copyfile('target/wasm32-unknown-unknown/release/varidor.wasm', 'html/varidor.wasm')
call(['wasm-gc', 'html/varidor.wasm', 'html/varidor-gc.wasm'])
