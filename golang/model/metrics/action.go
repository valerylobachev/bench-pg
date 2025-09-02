package metrics

import "bench-pg-go/model/domain"

type Action interface {
	Type() string
}

type ProcessYear struct{}

func NewProcessYear() ProcessYear {
	return ProcessYear{}
}

func (ProcessYear) Type() string {
	return "PROCESS_YEAR"
}

type ProcessPeriod struct{}

func NewProcessPeriod() ProcessPeriod {
	return ProcessPeriod{}
}

func (ProcessPeriod) Type() string {
	return "PROCESS_PERIOD"
}

type Operation struct {
	Operation domain.Operation
}

func NewOperationAction(operation domain.Operation) Operation {
	return Operation{operation}
}

func (o Operation) Type() string {
	return o.Operation.Code()
}
