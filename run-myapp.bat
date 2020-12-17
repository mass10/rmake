@SETLOCAL
@CALL yarn workspace my-lib tsc --build
@CALL yarn workspace my-app tsc --build
@CALL yarn node node_modules\my-app\dist\Main.js
