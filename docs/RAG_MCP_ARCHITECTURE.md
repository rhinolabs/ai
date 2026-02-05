# RAG MCP Architecture Plan

## Estado Actual (Problema)

### El MCP Worker No Existe

El código referencia un Cloudflare Worker en `https://rhinolabs-rag-mcp.rhinolabs.workers.dev/mcp` que **nunca fue creado**. El commit `41839ff` agregó toda la infraestructura de RAG asumiendo que este Worker existiría, pero:

1. No hay código del Worker en el repositorio
2. La URL devuelve 404
3. El código del CLI y core referencian esta URL hardcodeada

### URLs Hardcodeadas

La URL del MCP Worker está hardcodeada en 3 lugares:

| Archivo | Línea | Uso |
|---------|-------|-----|
| `core/src/rag.rs` | 12 | `DEFAULT_MCP_URL` constante |
| `cli/src/commands/rag.rs` | 15 | `DEFAULT_MCP_URL` constante |
| `rhinolabs-claude/.mcp.json` | 11 | Config estático del plugin |

### Por Qué Esto Es Un Problema

1. **No funciona**: El Worker no existe, por lo tanto ninguna funcionalidad RAG funciona
2. **No es configurable**: Los usuarios no pueden apuntar a su propio Worker
3. **Violación de arquitectura**: La URL debería venir de configuración, no de código
4. **Deploy de config estático**: El `.mcp.json` se copiaba durante instalación con la URL rota

---

## Arquitectura Propuesta

### Principio: La GUI Es La Fuente de Verdad

```
┌─────────────────────────────────────────────────────────────┐
│                         GUI                                  │
│  (Settings → MCP Servers → Add "rhinolabs-rag" HTTP server) │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              ~/.config/rhinolabs-ai/.mcp.json               │
│  {                                                          │
│    "mcpServers": {                                          │
│      "rhinolabs-rag": {                                     │
│        "url": "https://YOUR-WORKER.workers.dev/mcp",        │
│        "transport": "http"                                  │
│      }                                                      │
│    }                                                        │
│  }                                                          │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
         ┌────────┐     ┌─────────┐     ┌──────────┐
         │  CLI   │     │ Claude  │     │  Core    │
         │        │     │  Code   │     │          │
         └────────┘     └─────────┘     └──────────┘
```

### Por Qué La GUI

1. **Ya existe**: La GUI ya tiene una página MCP completa para gestionar servers
2. **Soporta HTTP**: Ya soporta servers HTTP con `url` y `transport: "http"`
3. **Persistencia**: Guarda en `McpConfigManager` que el CLI ya puede leer
4. **UI amigable**: El usuario no necesita editar JSON manualmente

### Por Qué NO Hardcodear

1. **Cada organización puede tener su propio Worker**: Rhinolabs, clientes, etc.
2. **Desarrollo local**: El Worker puede correr en localhost durante desarrollo
3. **Múltiples ambientes**: staging, production, etc.
4. **El código no debería asumir infraestructura**: Es responsabilidad de la configuración

---

## Plan de Implementación

### Fase 1: Dejar de Deployar Config Estático (COMPLETADO)

**Por qué**: El `.mcp.json` del plugin tiene una URL rota hardcodeada.

**Cambios**:
- [x] `install.sh`: Eliminar copia de `.mcp.json` a config del usuario
- [x] `install.ps1`: Eliminar copia de `.mcp.json` a config del usuario
- [x] Documentar que MCP se configura via GUI

### Fase 2: Modificar CLI Para Leer de Config

**Por qué**: El CLI actualmente usa `DEFAULT_MCP_URL` hardcodeado. Debería leer de la misma config que la GUI escribe.

**Cambios necesarios**:

1. **`core/src/rag.rs`**:
   ```rust
   // ANTES
   const DEFAULT_MCP_URL: &str = "https://rhinolabs-rag-mcp.rhinolabs.workers.dev";

   // DESPUÉS
   // Eliminar constante, leer de McpConfigManager
   pub fn get_mcp_url() -> Result<String> {
       let server = McpConfigManager::get_server("rhinolabs-rag")?;
       match server {
           Some(s) if s.is_http() => Ok(s.url.unwrap()),
           _ => Err(RhinolabsError::ConfigError(
               "RAG MCP server not configured. Add 'rhinolabs-rag' in GUI → MCP Servers".into()
           ))
       }
   }
   ```

2. **`cli/src/commands/rag.rs`**:
   ```rust
   // ANTES
   const DEFAULT_MCP_URL: &str = "https://rhinolabs-rag-mcp.rhinolabs.workers.dev";
   let mcp_url = env::var("RHINOLABS_RAG_MCP_URL").unwrap_or_else(|_| DEFAULT_MCP_URL.to_string());

   // DESPUÉS
   // Usar Rag::get_mcp_url() que lee de config
   // Mantener env var como override para desarrollo
   let mcp_url = env::var("RHINOLABS_RAG_MCP_URL")
       .or_else(|_| Rag::get_mcp_url())
       .map_err(|e| anyhow::anyhow!(e))?;
   ```

3. **`RagSettings`**: Eliminar `default_mcp_url` ya que la URL viene de MCP config, no de RAG settings.

### Fase 3: Crear el Cloudflare Worker

**Por qué**: Sin el Worker, el sistema RAG no funciona. Es el backend que:
- Almacena documentos en R2
- Indexa vectores con AutoRAG/Vectorize
- Responde a queries MCP

**Estructura del Worker**:

```
rhinolabs-rag-worker/
├── wrangler.toml          # Config de Cloudflare
├── src/
│   ├── index.ts           # Entry point
│   ├── mcp/
│   │   ├── handler.ts     # MCP protocol handler
│   │   └── tools.ts       # rag_save, rag_search, rag_ai_search
│   ├── storage/
│   │   └── r2.ts          # R2 bucket operations
│   ├── search/
│   │   └── vectorize.ts   # Vector search operations
│   └── auth/
│       └── keys.ts        # API key validation
└── package.json
```

**Endpoints MCP**:

| Tool | Descripción | Parámetros |
|------|-------------|------------|
| `rag_save` | Guardar documento | `project_id`, `content`, `metadata` |
| `rag_search` | Búsqueda por similitud vectorial | `project_id`, `query`, `limit` |
| `rag_ai_search` | Búsqueda + respuesta generada | `project_id`, `query` |

**Endpoints Admin** (para CLI):

| Endpoint | Método | Descripción |
|----------|--------|-------------|
| `/admin/keys` | POST | Crear API key |
| `/admin/keys` | GET | Listar API keys |
| `/admin/keys/:id` | DELETE | Eliminar API key |

### Fase 4: Actualizar GUI Para RAG

**Por qué**: Facilitar la configuración del RAG MCP server.

**Opciones**:

1. **Mínima**: Documentar que el usuario debe agregar manualmente en MCP Servers
2. **Mejor**: Agregar botón "Add Rhinolabs RAG" que pre-llene el formulario
3. **Óptima**: Página dedicada de RAG settings con validación de conexión

### Fase 5: Documentación

**Por qué**: Los usuarios necesitan saber cómo configurar y usar el sistema.

**Documentos**:
- `docs/RAG_SETUP.md`: Guía de configuración inicial
- `docs/RAG_WORKER_DEPLOYMENT.md`: Cómo deployar tu propio Worker
- Actualizar `README.md` con sección RAG funcional

---

## Decisiones de Diseño

### ¿Por qué Cloudflare Workers?

1. **Edge computing**: Baja latencia global
2. **R2**: Storage S3-compatible incluido
3. **Vectorize**: Vector search nativo (o AutoRAG)
4. **KV**: Key-value store para API keys
5. **Pricing**: Generous free tier para desarrollo

### ¿Por qué MCP en lugar de API REST directa?

1. **Integración nativa**: Claude Code ya soporta MCP
2. **Tools**: Los tools MCP aparecen automáticamente en Claude
3. **Context**: MCP permite pasar contexto adicional
4. **Estándar**: Anthropic está empujando MCP como estándar

### ¿Por qué config en GUI y no en CLI?

1. **Visualización**: Ver todos los MCP servers en un lugar
2. **Validación**: La GUI puede validar el formato
3. **Consistencia**: Una sola fuente de verdad
4. **UX**: Más amigable que editar JSON

---

## Preguntas Abiertas

1. **¿Self-hosted o managed?**: ¿Rhinolabs hostea el Worker o cada organización el suyo?
2. **¿Autenticación?**: API keys, OAuth, o ambos?
3. **¿Multi-tenant?**: ¿Un Worker para todos o uno por organización?
4. **¿Backup de datos?**: ¿Cómo se exportan/importan los datos de RAG?

---

## Archivos Afectados (Resumen)

| Archivo | Cambio |
|---------|--------|
| `rhinolabs-claude/scripts/install.sh` | No deployar `.mcp.json` |
| `rhinolabs-claude/scripts/install.ps1` | No deployar `.mcp.json` |
| `core/src/rag.rs` | Leer URL de McpConfigManager |
| `cli/src/commands/rag.rs` | Leer URL de McpConfigManager |
| `rhinolabs-claude/.mcp.json` | Eliminar `rhinolabs-rag` entry |
| `docs/INSTALLATION.md` | Actualizar sección MCP |
| `docs/RAG_SETUP.md` | NUEVO: Guía de configuración |
| `rhinolabs-rag-worker/` | NUEVO: Código del Worker |

---

**Última actualización**: 2026-02-06
