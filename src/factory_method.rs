pub trait Container {
    fn pull_from_registry(&self) {}
    fn describe(&self) {}
}

pub struct DockerContainer {
    img: String,
    registry: String
        
}

pub struct SingularityContainer {
    img: String,
    registry: String
}

impl Container for DockerContainer {
    fn pull_from_registry(&self) {
        println!("Pulling {} from {}", self.img, self.registry);
    }
}

impl Container for SingularityContainer {
    fn pull_from_registry(&self) {
        println!("Pulling {} from {}", self.img, self.registry);
    }
}

pub trait ContainerFactory {
    type Object;

    fn create(_img: &str) -> Box<Self::Object> { unimplemented!() }
}

pub struct DockerFactory{}

impl ContainerFactory for DockerFactory {
    type Object = DockerContainer;
    
    fn create(img: &str) -> Box<Self::Object> {
        Box::new(DockerContainer{
            img: img.to_string(),
            registry: "https://hub.docker.com/".to_string()
        })
    }
}

pub struct SingularityFactory{}

impl ContainerFactory for SingularityFactory {
    type Object = SingularityContainer;
    
    fn create(img: &str) -> Box<Self::Object> {
        Box::new(SingularityContainer{
            img: img.to_string(),
            registry: "https://singularityhub.com".to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default() {
        let container = DockerFactory::create("hello-world");
        container.pull_from_registry();
        assert!(container.img.contains("hello"));
    }
}
