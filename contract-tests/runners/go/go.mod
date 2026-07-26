module lifechronicle.local/contract-runner-go

go 1.26.0

require (
	github.com/lifechronicle/lifechronicle/gen/go v0.0.0
	google.golang.org/protobuf v1.36.11
)

replace github.com/lifechronicle/lifechronicle/gen/go => ../../../generated/go
