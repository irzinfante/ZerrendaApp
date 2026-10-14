# Convenciones de código de `app`

> Homogeneidad extrema. La IA predice mejor cuando el repositorio se parece
> a sí mismo en todas partes.

## Estilo TypeScript

- **Versión:** TypeScript 5 en modo `strict`, React 18. `npm run build` y
  `npm run lint` (`strictTypeChecked` + `stylisticTypeChecked`) deben pasar
  sin errores.
- **Formato:** indentación de 2 espacios, sin punto y coma.
- **Imports:** paquetes externos primero, luego módulos locales con rutas
  relativas y sin extensión; el `.css` propio del componente, al final.
- **Strings:** comillas dobles `"..."` siempre.
- **Template literals** para interpolación. Nada de concatenar con `+`.
- **Tipos:** nada de `any`, `as` para silenciar tipos, `// @ts-ignore` ni
  `eslint-disable`. `useState` con el genérico explícito
  (`useState<boolean>(false)`).
- **Textos:** los de la interfaz y los comentarios, en inglés.
- Nada de `console.log` en el código entregado.

## Estilo CSS

- **Paleta:** vive solo en `:root` de `index.css`, como variables
  `--<color>`. Fuera de ahí no hay colores literales: siempre `var(--…)`.
- **Componentes `ui/`:** estilos en `theme.ts` usando `var(--…)`, sin `.css`
  ni `className` propios.
- **Resto de componentes:** `<Nombre>.css` junto al `.tsx`, importado solo
  desde él.
- **`index.css`:** paleta y estilos globales de elementos HTML.
- **Responsive:** mobile first, con `@media (min-width: …)` en los
  breakpoints `601px`, `769px` y `1200px`.

## Nombres

| Tipo                       | Convención                    | Ejemplo                     |
|----------------------------|-------------------------------|-----------------------------|
| Componentes                | `PascalCase`, carpeta propia  | `Footer/Footer.tsx`         |
| Componentes `ui/`          | `PascalCase`, carpeta propia  | `ui/Button/Button.tsx`      |
| Hooks de componente `ui/`  | `use<Nombre>.tsx`             | `hooks/ui/useTextInput.tsx` |
| Hooks de dominio           | `use<Dominio>.ts`             | `hooks/useAuth.ts`          |
| Servicios                  | `<dominio>Service.ts`         | `services/authService.ts`   |
| Archivos de `constants/`   | `PascalCase`                  | `Environment.ts`            |
| Props                      | `<Nombre>Props`               | `ButtonProps`               |
| Tipo de un hook            | `Use<Nombre>`                 | `UseTextInput`              |
| Funciones / variables      | `camelCase`                   | `emailValue`                |
| Booleanos                  | prefijo `is`                  | `isDisabled`, `isInvalid`   |
| Manejadores de eventos     | prefijo `handle`              | `handleClick`               |
| Envoltorio de Chakra       | alias `ChakraUi<Nombre>`      | `ChakraUiButton`            |
| Variables de entorno       | `UPPER_SNAKE` sin `VITE_`     | `GITHUB_TAG`                |
| Patrones de validación     | `<campo>Pattern`              | `emailPattern`              |
| Clases e ids CSS           | `kebab-case`, prefijo del componente (salvo las clases globales de `index.css`) | `footer-content` |
| Variables CSS              | `--<color>` en `kebab-case`   | `--striped-blue`            |
| Tipos del contrato con `api` | `<Acción>Request` / `<Acción>Response`, campos en `snake_case` | `LoginResponse` |

## Estructura de archivo

Los componentes son funciones flecha con sus props en una `interface`
encima, desestructuradas en la firma, y `export default` al final. Si el
cuerpo es solo JSX, retorno implícito:

```tsx
import { Button as ChakraUiButton } from "@chakra-ui/react"

interface ButtonProps {
  text: string
  isDisabled: boolean
  handleClick: React.MouseEventHandler<HTMLButtonElement>
}

const Button = ({ text, isDisabled, handleClick }: ButtonProps) => (
  <ChakraUiButton isDisabled={isDisabled} onClick={handleClick}>
    {text}
  </ChakraUiButton>
)

export default Button
```

Los hooks de componentes `ui/` declaran su tipo `Use<Nombre>` y devuelven el
elemento bajo la clave `<Nombre>` junto con su estado:

```tsx
import React, { useState } from "react"
import TextInput from "../../components/ui/TextInput/TextInput"

type UseTextInput = (
  label: string,
  placeholder: string,
  validationRegex: RegExp
) => {
  TextInput: React.JSX.Element
  value: string
  isInvalid: boolean
}

const useTextInput: UseTextInput = (label, placeholder, regex) => {
  const [value, setValue] = useState<string>("")
  ...
  return { TextInput: <TextInput ... />, value, isInvalid }
}

export default useTextInput
```

Quien lo usa renombra al desestructurar:
`const { TextInput: EmailInput, value: emailValue } = useTextInput(...)`.

## Manejo de errores

El cliente base de `services/` tipa las respuestas con el formato de `api`:

```ts
interface ApiError {
  type: "ValidationError" | "AuthError" | "InternalError"
  message: string
}

interface ApiResponse<T> {
  success: boolean
  error: ApiError | null
  data: T | null
}
```

- La UI decide por código HTTP / `error.type`, nunca por el texto de
  `message`; el mensaje que ve el usuario lo pone la app.
- Los errores se guardan en estado del hook (`is<Algo>Error` o un mensaje) y
  la pantalla los muestra; nunca se tragan con un `catch` vacío.
- `throw new Error(...)` solo para invariantes del arranque (p. ej. el
  `#root` de `main.tsx`).

## Comentarios

Por defecto **no** se escriben. Solo se permiten cuando explican un *por qué*
no obvio (p. ej. por qué un estilo vive en `theme.ts` y no en un `.css`). Los
nombres deben hacer el resto.
