-- name: CRYPTIC
-- uses a mysterious pattern based on round number

function decide(round, history)
    if round % 7 == 0 then
        return "defect"
    else
        return "cooperate"
    end
end
