import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    kotlin("jvm") version "2.2.0"
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(":eventsJava"))
    implementation("com.google.protobuf:protobuf-java:4.35.0")
    implementation("com.google.protobuf:protobuf-kotlin:4.35.0")
}

sourceSets {
    main {
        kotlin.setSrcDirs(
            listOf(
                projectDir,
                file("../../../generated/kotlin"),
            ),
        )
        kotlin.include(
            "Phase0Runner.kt",
            "io/lifechronicle/events/v1/EventEnvelopeKt.kt",
            "io/lifechronicle/events/v1/AppForegroundKt.kt",
        )
    }
}

kotlin {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_17)
    }
}

application {
    mainClass.set("Phase0RunnerKt")
}

tasks.named<JavaExec>("run") {
    providers.gradleProperty("vectorPath").orNull?.let { args(it) }
}

tasks.withType<JavaCompile>().configureEach {
    options.encoding = "UTF-8"
    options.release.set(17)
}
