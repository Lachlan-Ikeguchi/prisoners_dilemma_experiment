-- name: DETECTIVE
-- probes opponent type then adapts strategy

function decide(round, history)
    if round == 1 then return "cooperate" end
    if round == 2 then return "defect" end
    if round == 3 then return "cooperate" end
    if round == 4 then return "cooperate" end
    
    -- After probing, classify opponent
    if #history >= 4 then
        -- If opponent cooperated on rounds 2-4, they're nice
        if history[1] == "cooperate" and history[2] == "cooperate" and history[3] == "cooperate" then
            return "cooperate"
        end
        -- If opponent defected on round 2 but cooperated on 3-4, they're TFT-like
        if history[1] == "defect" and history[2] == "cooperate" and history[3] == "cooperate" then
            return "cooperate"
        end
    end
    
    -- Default to Tit-for-Tat
    return history[#history]
end
