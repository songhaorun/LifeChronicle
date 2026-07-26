plugins {
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.google.protobuf:protobuf-java:4.35.0")
}

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

sourceSets {
    main {
        java.setSrcDirs(
            listOf(
                projectDir,
                file("../../../generated/java"),
            ),
        )
    }
}

application {
    mainClass.set("Phase0Runner")
}

tasks.named<JavaExec>("run") {
    providers.gradleProperty("vectorPath").orNull?.let { args(it) }
}

tasks.withType<JavaCompile>().configureEach {
    options.encoding = "UTF-8"
    options.release.set(17)
}
