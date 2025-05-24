package gorm_ex

import (
	"bench-pg-go/executors/gorm_ex/model"
	"bench-pg-go/model/domain"
	"gorm.io/gorm"
)

func openPeriod(db *gorm.DB, period domain.Period, user domain.User) {

	userId := user.Id()

	materials := make([]model.MaterialEntity, 0)
	res := db.Select("id").Find(&materials)
	if res.Error != nil {
		panic(res.Error)
	}

	for _, material := range materials {
		materialId := material.Id

		var cnt int64
		res = db.
			Model(&model.MaterialPeriodEntity{}).
			Where("material_id = ? and period = ?", materialId, period.YearPeriod()).
			Count(&cnt)
		if res.Error != nil {
			panic(res.Error)
		}
		if cnt == 0 {
			res := db.Exec(`insert into fin_material_periods
			                select material_id, ?,
			                       (select next_std_price from fin_materials where id = ?) as std_price,
			                       mov_avg_price, actual_price, sell_price, stock,
			                       ? as updated_by, now() as updated_at
			                from fin_material_periods
			                where material_id = ? and period = ?`,
				period.YearPeriod(),
				materialId,
				userId,
				materialId,
				period.PrevPeriod().YearPeriod(),
			)
			if res.Error != nil {
				panic(res.Error)
			}
		} else {
			res := db.Exec(`update fin_material_periods set
                      std_price = (select next_std_price from fin_materials where id = ?),
                      stock = (select stock from fin_material_periods where material_id = ? and period = ? ),
                      updated_by = ?, 
                      updated_at = now() 
                   where material_id = ? and period = ?`,
				materialId,
				materialId,
				period.PrevPeriod().YearPeriod(),
				userId,
				materialId,
				period.YearPeriod(),
			)
			if res.Error != nil {
				panic(res.Error)
			}

		}

	}
}
