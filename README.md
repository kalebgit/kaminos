# kaminos
cli program to create apis just with yml configuration files

### Project configuration
```yaml
# apiforge.yaml (archivo de configuración principal)
project:
  name: "TableTopRPG"
  organization: "domain" # o "entity"
  
domains:
  - users
  - characters  
  - combat
  - game_instances

# Configuración de resolución de referencias
resolution:
  auto_discover: true  # Busca automáticamente entidades
  explicit_imports: false  # Requiere imports explícitos
```

### Chossing between generation modes
```sh
# file per domain
apiforge generate --org=domain

# file per entity
apiforge generate --org=entity

```


### General Sintax

```yaml
entity_name: 
    lombok: 
        equals: all/explicitly_included
        to_string: none/include_field_names
        constructors: no_args/all #recordar callSuper para inheritance
        setters: all/explicitly_included
        getters: all/explicitly_included
          
    attribute: <primitive_type>
    #or
    attribute: 
        type: string
        required: true/false
        unique: true/false
        max_length: 50
        min_length: 3
        format: email
        nullable: false
        primary_key: true #includes nullable
            strategy::
        #para lombok
        lombok: 
            equals_included: true/false
            setter_access: public/private/protected/none
            getter_access: public/private/protected/none
            
        
    #for relationships between entities
    profile:
        type: one_to_one
        entity: user_profile
        # and for entities in other domains
        entity: posts/post
        foreign_key: user_id
        cascade: delete 


```
