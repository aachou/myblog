+++
title = "Building RESTful APIs with Java Spring Boot"
date = "2023-01-15"
tags = ["java", "spring-boot", "rest-api"]
excerpt = "A practical guide to building production-ready REST APIs using Spring Boot. Learn dependency injection, JPA repositories, and controller layering."
+++

Spring Boot has revolutionized Java web development by removing the boilerplate configuration that once plagued enterprise applications. With its auto-configuration and opinionated defaults, you can go from zero to a running web service in minutes.

## Setting Up a Project

The easiest way to bootstrap a Spring Boot project is via [Spring Initializr](https://start.spring.io). Select the dependencies you need — Web, JPA, H2, and Lombok — and generate the archive.

```bash
curl https://start.spring.io/starter.zip \
  -d dependencies=web,jpa,h2,lombok \
  -d javaVersion=17 \
  -o demo.zip
```

Unzip the archive and open it in your favorite IDE. The generated `pom.xml` includes all the transitive dependencies you need.

## Controller Layer

A REST controller maps HTTP requests to handler methods. Use the `@RestController` annotation combined with request-mapping annotations:

```java
@RestController
@RequestMapping("/api/v1/products")
public class ProductController {

    private final ProductService service;

    public ProductController(ProductService service) {
        this.service = service;
    }

    @GetMapping
    public List<Product> findAll() {
        return service.findAll();
    }

    @PostMapping
    @ResponseStatus(HttpStatus.CREATED)
    public Product create(@Valid @RequestBody Product product) {
        return service.save(product);
    }
}
```

| Annotation | Purpose |
|------------|---------|
| `@RestController` | Combines `@Controller` and `@ResponseBody` |
| `@RequestMapping` | Base URL mapping |
| `@GetMapping` | Maps HTTP GET |
| `@PostMapping` | Maps HTTP POST |

## Service Layer

Keep business logic out of controllers. A dedicated service bean handles transactions and validation:

```java
@Service
@Transactional
public class ProductService {

    private final ProductRepository repository;

    public ProductService(ProductRepository repository) {
        this.repository = repository;
    }

    public Product save(Product product) {
        if (repository.findBySku(product.getSku()).isPresent()) {
            throw new DuplicateSkuException(product.getSku());
        }
        return repository.save(product);
    }
}
```

## Repository Layer

Spring Data JPA eliminates repetitive data-access code. Extend `JpaRepository` and you get CRUD operations for free:

```java
public interface ProductRepository extends JpaRepository<Product, Long> {
    Optional<Product> findBySku(String sku);
    List<Product> findByCategoryId(Long categoryId);
}
```

## Exception Handling

Use `@ControllerAdvice` to centralize exception handling and return consistent error responses:

```java
@ControllerAdvice
public class GlobalExceptionHandler {

    @ExceptionHandler(DuplicateSkuException.class)
    @ResponseStatus(HttpStatus.CONFLICT)
    public ErrorResponse handleDuplicateSku(DuplicateSkuException ex) {
        return new ErrorResponse("CONFLICT", ex.getMessage());
    }
}
```

## Testing

Write integration tests with `@SpringBootTest` and mock the web layer with `MockMvc`:

```java
@SpringBootTest
@AutoConfigureMockMvc
class ProductControllerTest {

    @Autowired
    private MockMvc mockMvc;

    @Test
    void shouldCreateProduct() throws Exception {
        mockMvc.perform(post("/api/v1/products")
                .contentType(MediaType.APPLICATION_JSON)
                .content("{\"name\":\"Widget\",\"sku\":\"WID-001\"}"))
                .andExpect(status().isCreated());
    }
}
```

Spring Boot's convention-over-configuration approach lets you focus on business logic rather than infrastructure. Combined with its rich ecosystem of starters, it remains the top choice for Java microservices in 2023.
